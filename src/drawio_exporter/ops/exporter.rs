use anyhow::{Context, Result, anyhow};

use crate::core::drawio::drawio_desktop::{DrawioDesktop, ExportArguments};
use crate::core::drawio::mxfile::{Diagram, Mxfile};
use crate::core::explorer::filesystem;
use crate::core::explorer::filesystem::FilterOptions;
use crate::core::explorer::git_repository;
use crate::ops::progress::{ExportEvent, ExportProgress};
use relative_path::RelativePath;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct ExporterOptions<'a> {
    pub application: &'a String,
    pub drawio_desktop_headless: bool,
    pub folder: &'a String,
    pub on_filesystem_changes: bool,
    pub on_git_changes_since_reference: Option<&'a String>,
    pub remove_page_suffix: bool,
    pub path: &'a str,
    pub format: &'a String,
    pub border: &'a String,
    pub scale: Option<&'a String>,
    pub enable_plugins: bool,
    pub width: Option<&'a String>,
    pub height: Option<&'a String>,
    pub crop: bool,
    pub all_pages: bool,
    pub transparent: bool,
    pub quality: &'a String,
    pub uncompressed: bool,
    pub embed_svg_fonts: bool,
    pub embed_svg_images: bool,
    pub svg_theme: Option<&'a String>,
    pub svg_links_target: Option<&'a String>,
    pub embed_diagram: bool,
}

/// Sanitizes a diagram name to be filesystem-safe by replacing reserved characters with hyphens.
fn sanitize_diagram_name(name: &str) -> String {
    name.chars()
        .map(|x| match x {
            ' ' => '-',
            // https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words
            '&' => '-',
            '#' => '-',
            '%' => '-',
            '{' => '-',
            '}' => '-',
            '\\' => '-',
            '/' => '-',
            '<' => '-',
            '>' => '-',
            '*' => '-',
            '?' => '-',
            '$' => '-',
            '!' => '-',
            '\'' => '-',
            '"' => '-',
            ':' => '-',
            ';' => '-',
            ',' => '-',
            '@' => '-',
            '+' => '-',
            '`' => '-',
            '|' => '-',
            '=' => '-',
            _ => x,
        })
        .collect::<String>()
}

/// Resolves the actual export format to use. Maps documentation formats (adoc, md) to PNG.
fn resolve_export_format(format: &str) -> &str {
    match format {
        "adoc" | "md" => "png",
        _ => format,
    }
}

/// Determines whether to include page suffix in output filenames.
fn should_include_page_suffix(remove_page_suffix: bool, diagram_count: usize) -> bool {
    !(remove_page_suffix && diagram_count == 1)
}

/// Builds the output path for an exported file.
fn build_output_path(base_path: &Path, folder: &str, filename: &str) -> PathBuf {
    base_path.parent().unwrap().join(folder).join(filename)
}

/// Builds export arguments from the exporter options.
fn build_export_arguments<'a>(
    options: &'a ExporterOptions,
    input: &'a str,
    output: Option<&'a str>,
    format: &'a str,
    page_index: Option<&'a String>,
) -> ExportArguments<'a> {
    ExportArguments {
        recursive: false,
        output,
        input,
        format,
        border: options.border,
        scale: options.scale,
        width: options.width,
        height: options.height,
        crop: options.crop,
        embed_diagram: options.embed_diagram,
        transparent: options.transparent,
        quality: options.quality,
        uncompressed: options.uncompressed,
        all_pages: options.all_pages,
        page_index,
        page_range: None,
        embed_svg_fonts: options.embed_svg_fonts,
        embed_svg_images: options.embed_svg_images,
        svg_theme: options.svg_theme,
        svg_links_target: options.svg_links_target,
        enable_plugins: options.enable_plugins,
    }
}

pub fn exporter(options: ExporterOptions<'_>, progress: &mut dyn ExportProgress) -> Result<()> {
    // Fallback in case of an empty path, we take the current directory
    let input_path = match options.path {
        "" => PathBuf::from("."),
        path => PathBuf::from(path),
    };
    if !input_path.exists() {
        return Err(anyhow!(format!(
            "path '{}' must exist (as directory or file)",
            options.path
        )));
    }

    let drawio_files = match options.on_git_changes_since_reference {
        None => {
            let filter_options = match options.on_filesystem_changes {
                true => FilterOptions::filter_on(options.folder),
                false => FilterOptions::no_filtering(),
            };
            filesystem::explore_path(&input_path, filter_options)
        }
        Some(git_reference) => git_repository::explore_path(&input_path, git_reference),
    }
    .with_context(|| format!("can't explore path {}", &input_path.display()))?;

    let drawio_desktop = DrawioDesktop::new(options.application, options.drawio_desktop_headless)?;

    prepare_export_folders(options.folder, &drawio_files)
        .with_context(|| format!("can't prepare export folders named {}", options.folder))?;

    let total_files = drawio_files.len();
    progress.on_event(ExportEvent::ExportStart { total_files });

    let drawio_path_base = RelativePath::new(options.path);
    for (file_index, (path, mxfile)) in drawio_files.into_iter().enumerate() {
        let drawio_file_path = drawio_path_base.relative(RelativePath::new(path.to_str().unwrap()));
        progress.on_event(ExportEvent::FileStart {
            path: drawio_file_path.as_ref(),
            file_index,
            total_files,
        });

        // If 'all pages' option is set and the format is PDF, we export all pages at once
        if is_pdf_all_pages_enabled(&options) || is_xml_format_enabled(&options) {
            export_pdf_all_pages(&options, &drawio_desktop, &path, progress)?;
        } else {
            export_per_page(&options, &drawio_desktop, &path, mxfile, progress)?;
        }

        progress.on_event(ExportEvent::FileComplete);
    }

    progress.on_event(ExportEvent::ExportComplete);
    Ok(())
}

fn is_pdf_all_pages_enabled(options: &ExporterOptions) -> bool {
    options.all_pages && options.format == "pdf"
}

fn is_xml_format_enabled(options: &ExporterOptions) -> bool {
    options.format == "xml"
}

fn export_per_page(
    options: &ExporterOptions,
    drawio_desktop: &DrawioDesktop,
    path: &Path,
    mxfile: Mxfile,
    progress: &mut dyn ExportProgress,
) -> Result<()> {
    let total_pages = mxfile.diagrams.len();
    let with_page_suffix = should_include_page_suffix(options.remove_page_suffix, total_pages);
    for (position, diagram) in mxfile.diagrams.iter().enumerate() {
        let position_to_use = position + 1;
        let valid_diagram_name = sanitize_diagram_name(&diagram.name);
        progress.on_event(ExportEvent::PageStart {
            page_index: position_to_use,
            page_name: &valid_diagram_name,
            total_pages,
        });

        let file_stem = path.file_stem().unwrap();
        let file_stem_suffix = match with_page_suffix {
            true => format!("-{}", valid_diagram_name),
            false => "".to_string(),
        };
        let real_format = resolve_export_format(options.format.as_str());
        let output_filename = format!(
            "{}{}.{}",
            file_stem.to_str().unwrap(),
            file_stem_suffix,
            real_format
        );
        let output_path = build_output_path(path, options.folder, &output_filename);

        progress.on_event(ExportEvent::GenerateFile {
            format: real_format,
        });

        let page_index_str = position_to_use.to_string();
        drawio_desktop.execute(build_export_arguments(
            options,
            path.to_str().unwrap(),
            output_path.to_str(),
            real_format,
            Some(&page_index_str),
        ))?;

        if options.format.eq("adoc") || options.format.eq("md") {
            generate_formatted_text_file(
                options,
                path,
                diagram,
                file_stem,
                file_stem_suffix,
                output_filename,
                progress,
            )?;
        }
    }
    Ok(())
}

fn export_pdf_all_pages(
    options: &ExporterOptions,
    drawio_desktop: &DrawioDesktop,
    path: &Path,
    progress: &mut dyn ExportProgress,
) -> Result<()> {
    progress.on_event(ExportEvent::AllPagesStart);
    progress.on_event(ExportEvent::GenerateFile {
        format: options.format.as_str(),
    });

    let file_stem = path.file_stem().unwrap();
    let output_filename = format!(
        "{}.{}",
        file_stem.to_str().unwrap(),
        options.format.as_str()
    );
    let output_path = build_output_path(path, options.folder, &output_filename);

    drawio_desktop.execute(build_export_arguments(
        options,
        path.to_str().unwrap(),
        output_path.to_str(),
        options.format.as_str(),
        None,
    ))
}

fn generate_formatted_text_file(
    options: &ExporterOptions<'_>,
    path: &Path,
    diagram: &Diagram,
    file_stem: &OsStr,
    file_stem_suffix: String,
    output_filename: String,
    progress: &mut dyn ExportProgress,
) -> Result<()> {
    progress.on_event(ExportEvent::GenerateDocFile {
        format: options.format,
    });
    let formatted_text_filename = format!(
        "{}{}.{}",
        file_stem.to_str().unwrap(),
        file_stem_suffix,
        options.format
    );
    let formatted_text_path = build_output_path(path, options.folder, &formatted_text_filename);

    let mut file = File::create(formatted_text_path)?;
    if options.format.eq("adoc") {
        write!(
            file,
            "= {} {}

image::{}[{}]

",
            file_stem.to_str().unwrap(),
            diagram.name,
            output_filename,
            diagram.name
        )?;
    } else if options.format.eq("md") {
        write!(
            file,
            "# {} {}

![{}][{}]

",
            file_stem.to_str().unwrap(),
            diagram.name,
            diagram.name,
            output_filename,
        )?;
    }

    progress.on_event(ExportEvent::IncludeLinks {
        format: options.format,
    });
    for (link, label) in diagram.get_links() {
        if label.is_empty() {
            progress.on_event(ExportEvent::LinkWarning {
                message: format!(
                    "warn: link not included, due to missing label: link '[missing]' to {}",
                    link
                ),
            });
            continue;
        }
        if link.is_empty() {
            progress.on_event(ExportEvent::LinkWarning {
                message: format!(
                    "warn: link not included, due to missing url: link '{}' to [missing]",
                    label
                ),
            });
            continue;
        }
        if link.starts_with("data:page/id") {
            progress.on_event(ExportEvent::LinkWarning {
                message: format!(
                    "warn: link not included, page link isn't supported, link '{}' to {}",
                    label, link
                ),
            });
            continue;
        }
        progress.on_event(ExportEvent::LinkIncluded {
            label: &label,
            url: &link,
        });

        if options.format.eq("adoc") {
            // Since asciidoc consider '--' string as 'Em dash' string,
            // we need to protect it in order to be usable.
            writeln!(file, "* {}[{}]", link.replace("--", "\\--"), label)?;
        } else if options.format.eq("md") {
            writeln!(file, "* [{}]({})", label, link)?;
        }
    }
    Ok(())
}

fn prepare_export_folders(folder: &str, drawio_files: &[(PathBuf, Mxfile)]) -> Result<()> {
    let parent_paths: Vec<PathBuf> = drawio_files
        .iter()
        .map(|(path, _)| path.parent().unwrap().to_path_buf())
        .collect();
    for parent_path in parent_paths {
        fs::create_dir_all(parent_path.join(folder)).with_context(|| {
            format!(
                "can't prepare export folder named {} in path {}",
                folder,
                parent_path.display()
            )
        })?;
    }
    Ok(())
}

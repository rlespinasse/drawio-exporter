use anyhow::{anyhow, Context, Result};

use crate::core::drawio::drawio_desktop::{DrawioDesktop, ExportArguments};
use crate::core::drawio::mxfile::{Diagram, Mxfile};
use crate::core::explorer::filesystem;
use crate::core::explorer::filesystem::FilterOptions;
use crate::core::explorer::git_repository;
use relative_path::RelativePath;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct ExporterOptions<'a> {
    pub application: Option<&'a str>,
    pub drawio_desktop_headless: bool,
    pub folder: &'a str,
    pub on_filesystem_changes: bool,
    pub on_git_changes_since_reference: Option<&'a str>,
    pub remove_page_suffix: bool,
    pub path: &'a str,
    pub format: &'a str,
    pub border: &'a str,
    pub scale: Option<&'a str>,
    pub enable_plugins: bool,
    pub width: Option<&'a str>,
    pub height: Option<&'a str>,
    pub crop: bool,
    pub transparent: bool,
    pub quality: &'a str,
    pub uncompressed: bool,
    pub embed_svg_images: bool,
    pub embed_diagram: bool,
}

pub fn exporter(options: ExporterOptions) -> Result<()> {
    // Fallback in case of empty path, we take the current directory
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

    let drawio_path_base = RelativePath::new(options.path);
    for (path, mxfile) in drawio_files {
        let drawio_file_path = drawio_path_base.relative(RelativePath::new(path.to_str().unwrap()));
        println!("+ export file : {}", drawio_file_path);
        let with_page_suffix = !(options.remove_page_suffix && mxfile.diagrams.len() == 1);
        for (position, diagram) in mxfile.diagrams.iter().enumerate() {
            let position_to_display = position + 1;
            println!("++ export page {} : {}", position_to_display, diagram.name);

            let file_stem = path.file_stem().unwrap();
            let file_stem_suffix = match with_page_suffix {
                true => {
                    let page_suffix = diagram.name.replace(' ', "-");
                    format!("-{}", page_suffix)
                }
                false => "".to_string(),
            };
            let real_format = match options.format {
                "adoc" => "png",
                "md" => "png",
                _ => options.format,
            };
            let output_filename = format!(
                "{}{}.{}",
                file_stem.to_str().unwrap(),
                file_stem_suffix,
                real_format
            );
            let output_path = path
                .parent()
                .unwrap()
                .join(options.folder)
                .join(&output_filename);

            println!("+++ generate {} file", real_format);

            drawio_desktop.execute(ExportArguments {
                recursive: false,
                output: output_path.to_str(),
                input: path.to_str().unwrap(),
                format: real_format,
                border: options.border,
                scale: options.scale,
                width: options.width,
                height: options.height,
                crop: options.crop,
                embed_diagram: options.embed_diagram,
                transparent: options.transparent,
                quality: options.quality,
                uncompressed: options.uncompressed,
                all_pages: false,
                page_index: Some(position.to_string().as_str()),
                page_range: None,
                embed_svg_images: options.embed_svg_images,
                enable_plugins: options.enable_plugins,
            })?;

            if options.format.eq("adoc") || options.format.eq("md") {
                generate_formatted_text_file(
                    &options,
                    &path,
                    diagram,
                    file_stem,
                    file_stem_suffix,
                    output_filename,
                )?;
            }
        }
    }

    Ok(())
}

fn generate_formatted_text_file(
    options: &ExporterOptions,
    path: &Path,
    diagram: &Diagram,
    file_stem: &OsStr,
    file_stem_suffix: String,
    output_filename: String,
) -> Result<()> {
    println!("+++ generate {} file", options.format);
    let formatted_text_filename = format!(
        "{}{}.{}",
        file_stem.to_str().unwrap(),
        file_stem_suffix,
        options.format
    );
    let formatted_text_path = path
        .parent()
        .unwrap()
        .join(options.folder)
        .join(formatted_text_filename);

    let mut file = File::create(&formatted_text_path)?;
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

    println!("+++ include links in {} file", options.format);
    for (link, label) in diagram.get_links() {
        if label.is_empty() {
            println!(
                "warn: link not included, due to missing label: link '[missing]' to {}",
                link
            );
            continue;
        }
        if link.is_empty() {
            println!(
                "warn: link not included, due to missing url: link '{}' to [missing]",
                label
            );
            continue;
        }
        if link.starts_with("data:page/id") {
            println!(
                "warn: link not included, page link isn't supported, link '{}' to {}",
                label, link
            );
            continue;
        }
        println!("link '{}' to {}", label, link);

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

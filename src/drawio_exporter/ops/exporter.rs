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
    pub width: Option<&'a str>,
    pub height: Option<&'a str>,
    pub crop: bool,
    pub embed_diagram: bool,
    pub transparent: bool,
    pub quality: &'a str,
    pub uncompressed: bool,
}

pub fn exporter(options: ExporterOptions) -> Result<()> {
    let input_path = PathBuf::from(options.path);
    if !input_path.exists() {
        return Err(anyhow!("path must exist (as directory or file)"));
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
            println!("++ export page {} : {}", position + 1, diagram.name);

            let file_stem = path.file_stem().unwrap();
            let file_stem_suffix = match with_page_suffix {
                true => {
                    let page_suffix = diagram.name.replace(" ", "-");
                    format!("-{}", page_suffix)
                }
                false => "".to_string(),
            };
            let real_format = match options.format {
                "adoc" => "png",
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
                page_index: Some((position + 1).to_string().as_str()),
                page_range: None,
            })?;

            if options.format.eq("adoc") {
                generate_adoc_file(
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

fn generate_adoc_file(
    options: &ExporterOptions,
    path: &Path,
    diagram: &Diagram,
    file_stem: &OsStr,
    file_stem_suffix: String,
    output_filename: String,
) -> Result<()> {
    println!("+++ generate adoc file");
    let adoc_filename = format!("{}{}.adoc", file_stem.to_str().unwrap(), file_stem_suffix);
    let adoc_path = path
        .parent()
        .unwrap()
        .join(options.folder)
        .join(adoc_filename);

    let mut file = File::create(&adoc_path)?;
    write!(
        file,
        "= {} {}\n\nimage::{}[{}]\n\n",
        file_stem.to_str().unwrap(),
        diagram.name,
        output_filename,
        diagram.name
    )?;

    println!("+++ include links in adoc file");
    for (text, link) in diagram.get_links() {
        if text.is_empty() {
            println!("WARNING: text link is empty");
            continue;
        }
        if link.is_empty() {
            println!("WARNING: link is empty");
            continue;
        }
        if link.starts_with("data:page/id") {
            println!("WARNING: link between pages is not currently supported");
            continue;
        }
        println!("link '{}' : {}", text, link);
        // Since asciidoc consider '--' string as 'Em dash' string,
        // we need to protect it in order to be usable.
        writeln!(file, "* {}[{}]", link.replace("--", "\\--"), text)?;
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

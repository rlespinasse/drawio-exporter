use crate::core::drawio::mxfile::{read_file, Mxfile};
use anyhow::Result;
use ignore::WalkBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug)]
pub struct FilterOptions<'a> {
    on_changes: bool,
    folder: &'a str,
}

impl<'a> FilterOptions<'a> {
    pub fn no_filtering() -> FilterOptions<'a> {
        FilterOptions {
            on_changes: false,
            folder: "",
        }
    }

    pub fn filter_on(folder: &'a str) -> FilterOptions<'a> {
        FilterOptions {
            on_changes: true,
            folder,
        }
    }
}

pub fn explore_path(path: &Path, filter_options: FilterOptions<'_>) -> Result<Vec<(PathBuf, Mxfile)>> {
    let drawio_paths: Vec<PathBuf> = collect_files_from_filesystem(path);

    let mut files: Vec<(PathBuf, Mxfile)> = vec![];
    for drawio_path in drawio_paths {
        files.push((drawio_path.clone(), read_file(&drawio_path)?))
    }

    files.sort_by(|(a, _), (b, _)| a.cmp(b));

    match filter_options.on_changes {
        true => Ok(only_keep_changed_drawio_files(files, filter_options.folder)),
        false => Ok(files),
    }
}

fn collect_files_from_filesystem(path: &Path) -> Vec<PathBuf> {
    WalkBuilder::new(path)
        .build()
        .filter_map(|r| r.ok())
        .filter(|d| {
            d.path().is_file()
                && match d.path().extension() {
                    Some(ext) => ext.eq("drawio"),
                    None => false,
                }
        })
        .map(|d| d.into_path())
        .collect::<Vec<PathBuf>>()
}

fn only_keep_changed_drawio_files(
    files: Vec<(PathBuf, Mxfile)>,
    export_folder: &str,
) -> Vec<(PathBuf, Mxfile)> {
    files
        .into_iter()
        .filter(|(path, _)| is_drawio_file_has_been_updated(path, export_folder))
        .collect::<Vec<(PathBuf, Mxfile)>>()
}

fn is_drawio_file_has_been_updated(path: &Path, export_folder: &str) -> bool {
    match get_modified_date(path) {
        None => true, // If we can't access a modified date, we consider its have been updated
        Some(drawio_file_modified_date) => {
            is_drawio_file_older_than_exported_files(path, drawio_file_modified_date, export_folder)
        }
    }
}

fn is_drawio_file_older_than_exported_files(
    path: &Path,
    modified_date: SystemTime,
    export_folder: &str,
) -> bool {
    let export_folder_path = path.parent().unwrap().join(export_folder);
    let path_canonicalize = path.canonicalize().unwrap_or_default();
    let path_file_stem = path_canonicalize
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let existing_exported_files = WalkBuilder::new(export_folder_path)
        .build()
        .filter_map(|r| r.ok())
        .filter(|d| {
            let exported_path_canonicalize = d.path().canonicalize().unwrap_or_default();
            let exported_path_file_stem = exported_path_canonicalize
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            d.path().is_file() && exported_path_file_stem.starts_with(path_file_stem)
        })
        .map(|d| d.into_path())
        .collect::<Vec<PathBuf>>();

    if existing_exported_files.is_empty() {
        // So the drawio file is new, we need to export it
        return true;
    }

    let exported_files_older_than_drawio_file_count = existing_exported_files
        .iter()
        .filter(
            |&existing_exported_file| match get_modified_date(existing_exported_file) {
                // If we can't access a modified date, we consider that this file is older than the drawio file
                None => true,
                Some(exported_file_modified_date) => {
                    exported_file_modified_date.elapsed().unwrap().as_secs()
                        > modified_date.elapsed().unwrap().as_secs()
                }
            },
        )
        .count();
    exported_files_older_than_drawio_file_count > 0
}

fn get_modified_date(path: &Path) -> Option<SystemTime> {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified_date) = metadata.modified() {
            return Some(modified_date);
        }
    }
    None
}

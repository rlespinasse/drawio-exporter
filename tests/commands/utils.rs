use crate::DrawioExporterCommand;
use anyhow::{Error, anyhow};
use glob::glob;
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};

pub fn check_generate_files(
    drawio_exporter: &mut DrawioExporterCommand,
    format_to_search: &str,
    output_files: Vec<&str>,
) -> anyhow::Result<(), Error> {
    let search_pattern = format!(
        "{}/**/*.{}",
        drawio_exporter.current_dir.display(),
        format_to_search
    );

    let actual_files: HashSet<OsString> = glob(&search_pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .filter_map(|path| path.file_name().map(|s| s.to_os_string()))
        .collect();

    let expected_files: HashSet<OsString> = output_files
        .into_iter()
        .map(OsStr::new)
        .map(|s| s.to_os_string())
        .collect();

    if actual_files.is_disjoint(&expected_files) || expected_files.is_disjoint(&actual_files) {
        let missing_files: HashSet<_> = expected_files.difference(&actual_files).cloned().collect();
        let extra_files: HashSet<_> = actual_files.difference(&expected_files).cloned().collect();

        return Err(anyhow!(
            "File mismatch detected:\n\n\
            Actual files:\n{:#?}\n\n\
            Expected files:\n{:#?}\n\n\
            Missing files (expected but not found):\n{:#?}\n\n\
            Extra files (found but not expected):\n{:#?}",
            actual_files,
            expected_files,
            missing_files,
            extra_files
        ));
    }

    Ok(())
}

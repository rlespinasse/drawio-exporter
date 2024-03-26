use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use filetime::FileTime;
use predicates::prelude::predicate::str::contains;
use std::path::Path;
use std::time::Duration;

#[test]
fn export_only_changed_files() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("tree", false)?;

    let output_first_run = "+ export file : tree/file1.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page 2
\\ generate pdf file
+ export file : tree/folder1/file2.1.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page 2
\\ generate pdf file
+ export file : tree/folder1/file2.2.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page 2
\\ generate pdf file
+ export file : tree/folder2/folder3/file3.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page 2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output_first_run));

    // Update output file to simulate an older generated file
    // All exported files of tree/file1.drawio are older
    change_file_mtime(
        &drawio_exporter.current_dir,
        "tree/export/file1-Page-1.pdf",
        "tree/file1.drawio",
        7200,
    )?;
    change_file_mtime(
        &drawio_exporter.current_dir,
        "tree/export/file1-Page-2.pdf",
        "tree/file1.drawio",
        7200,
    )?;
    // Only one exported file of tree/folder1/file2.2.drawio is older
    change_file_mtime(
        &drawio_exporter.current_dir,
        "tree/folder1/export/file2.2-Page-2.pdf",
        "tree/folder1/file2.2.drawio",
        7200,
    )?;

    let output_second_run = "+ export file : tree/file1.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page 2
\\ generate pdf file
+ export file : tree/folder1/file2.2.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page 2
\\ generate pdf file";

    drawio_exporter.new_cmd()?;
    drawio_exporter
        .cmd
        .arg("--on-changes")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output_second_run));

    Ok(())
}

fn change_file_mtime(
    base_path: &Path,
    path_to_change: &str,
    based_on: &str,
    duration: u64,
) -> Result<()> {
    let modified_date = base_path.join(based_on).metadata()?.modified()?;
    let before_date = modified_date
        .checked_sub(Duration::new(duration, 0))
        .unwrap();
    filetime::set_file_mtime(
        base_path.join(path_to_change),
        FileTime::from_system_time(before_date),
    )?;

    Ok(())
}

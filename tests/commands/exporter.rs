use crate::DrawioExporterCommand;
use anyhow::{anyhow, Result};
use assert_cmd::prelude::*;
use glob::glob;
use predicates::prelude::predicate::str::contains;
use std::ffi::OsStr;

#[test]
fn export_nothing() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("empty_folder", true)?;

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(""); // No Output

    Ok(())
}

#[test]
fn export_files_with_a_name_collision() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("name_collision", true)?;

    let output = "+ export file : name_collision/name-collision.drawio
- export page 1 : Page-1
\\ generate pdf file
+ export file : name_collision/name.drawio
- export page 1 : Page-1
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_files_from_a_folders_tree() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("tree", true)?;

    let output = "+ export file : tree/file1.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file
+ export file : tree/folder1/file2.1.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file
+ export file : tree/folder1/file2.2.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file
+ export file : tree/folder2/folder3/file3.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_file_with_spaces() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("space", true)?;

    let output = "+ export file : space/file with spaces.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_file_with_illegal_characters() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("illegal-characters", true)?;

    let output = "+ export file : illegal-characters/names.drawio
- export page 1 : Page-pound
\\ generate pdf file
- export page 2 : Page-percent
\\ generate pdf file
- export page 3 : Page-ampersand
\\ generate pdf file
- export page 4 : Page-left-curly-bracket
\\ generate pdf file
- export page 5 : Page-right-curly-bracket
\\ generate pdf file
- export page 6 : Page-back-slash
\\ generate pdf file
- export page 7 : Page-left-angle-bracket
\\ generate pdf file
- export page 8 : Page-right-angle-bracket
\\ generate pdf file
- export page 9 : Page-asterisk
\\ generate pdf file
- export page 10 : Page-question-mark
\\ generate pdf file
- export page 11 : Page-forward-slash
\\ generate pdf file
- export page 12 : Page-dollar-sign
\\ generate pdf file
- export page 13 : Page-exclamation-point
\\ generate pdf file
- export page 14 : Page-single-quotes
\\ generate pdf file
- export page 15 : Page-double-quotes
\\ generate pdf file
- export page 16 : Page-colon
\\ generate pdf file
- export page 17 : Page-at-sign
\\ generate pdf file
- export page 18 : Page-plus-sign
\\ generate pdf file
- export page 19 : Page-backtick
\\ generate pdf file
- export page 20 : Page-pipe
\\ generate pdf file
- export page 21 : Page-equal-sign
\\ generate pdf file
- export page 22 : Page-semicolon
\\ generate pdf file
- export page 23 : Page-comma
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec![
        "names-Page-pound.pdf",
        "names-Page-percent.pdf",
        "names-Page-ampersand.pdf",
        "names-Page-left-curly-bracket.pdf",
        "names-Page-right-curly-bracket.pdf",
        "names-Page-back-slash.pdf",
        "names-Page-left-angle-bracket.pdf",
        "names-Page-right-angle-bracket.pdf",
        "names-Page-asterisk.pdf",
        "names-Page-question-mark.pdf",
        "names-Page-forward-slash.pdf",
        "names-Page-dollar-sign.pdf",
        "names-Page-exclamation-point.pdf",
        "names-Page-single-quotes.pdf",
        "names-Page-double-quotes.pdf",
        "names-Page-colon.pdf",
        "names-Page-at-sign.pdf",
        "names-Page-plus-sign.pdf",
        "names-Page-backtick.pdf",
        "names-Page-pipe.pdf",
        "names-Page-equal-sign.pdf",
        "names-Page-semicolon.pdf",
        "names-Page-comma.pdf",
    ];

    let os_output_files = output_files.iter().map(OsStr::new).collect::<Vec<&OsStr>>();

    let search_pattern = format!("{}/**/*.pdf", &drawio_exporter.current_dir.display());
    let missing_files: Vec<_> = glob(search_pattern.as_str())?
        .map(|entry| entry.unwrap())
        .filter(|entry| !os_output_files.contains(&entry.as_path().file_name().unwrap()))
        .collect();

    if !missing_files.is_empty() {
        return Err(anyhow!(format!("Missing files: {:#?}", missing_files)));
    }

    Ok(())
}

#[test]
fn export_file_using_shapes() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("shapes", true)?;

    let output = "+ export file : shapes/shapes.drawio
- export page 1 : AWS
\\ generate pdf file
- export page 2 : Azure
\\ generate pdf file
- export page 3 : GCP
\\ generate pdf file
- export page 4 : K8S
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_file_without_any_diagram() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("empty_file", true)?;

    let output = "+ export file : empty_file/empty.drawio";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_file_from_vscode() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("vscode", true)?;

    let output = "+ export file : vscode/vscode.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

use crate::DrawioExporterCommand;
use anyhow::{anyhow, Result};
use assert_cmd::prelude::*;
use ignore::WalkBuilder;
use predicates::prelude::predicate::str::contains;
use std::ffi::OsStr;
use std::path::PathBuf;

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
++ export page 1 : Page-1
+++ generate pdf file
+ export file : name_collision/name.drawio
++ export page 1 : Page-1
+++ generate pdf file";

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
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file
+ export file : tree/folder1/file2.1.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file
+ export file : tree/folder1/file2.2.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file
+ export file : tree/folder2/folder3/file3.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file";

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
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file";

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
++ export page 1 : Page#pound
+++ generate pdf file
++ export page 2 : Page%percent
+++ generate pdf file
++ export page 3 : Page&ampersand
+++ generate pdf file
++ export page 4 : Page{left-curly-bracket
+++ generate pdf file
++ export page 5 : Page}right-curly-bracket
+++ generate pdf file
++ export page 6 : Page\\back-slash
+++ generate pdf file
++ export page 7 : Page<left-angle-bracket
+++ generate pdf file
++ export page 8 : Page>right-angle-bracket
+++ generate pdf file
++ export page 9 : Page*asterisk
+++ generate pdf file
++ export page 10 : Page?question-mark
+++ generate pdf file
++ export page 11 : Page/forward-slash
+++ generate pdf file
++ export page 12 : Page$dollar-sign
+++ generate pdf file
++ export page 13 : Page!exclamation-point
+++ generate pdf file
++ export page 14 : Page'single-quotes
+++ generate pdf file
++ export page 15 : Page\"double-quotes
+++ generate pdf file
++ export page 16 : Page:colon
+++ generate pdf file
++ export page 17 : Page@at-sign
+++ generate pdf file
++ export page 18 : Page+plus-sign
+++ generate pdf file
++ export page 19 : Page`backtick
+++ generate pdf file
++ export page 20 : Page|pipe
+++ generate pdf file
++ export page 21 : Page=equal-sign
+++ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let curren_dir = drawio_exporter.current_dir.clone();

    let output_files = vec![
        "Page#pound.pdf",
        "Page%percent.pdf",
        "Page&ampersand.pdf",
        "Page{left-curly-bracket.pdf",
        "Page}right-curly-bracket.pdf",
        "Page\\back-slash.pdf",
        "Page<left-angle-bracket.pdf",
        "Page>right-angle-bracket.pdf",
        "Page*asterisk.pdf",
        "Page?question-mark.pdf",
        "Page/forward-slash.pdf",
        "Page$dollar-sign.pdf",
        "Page!exclamation-point.pdf",
        "Page'single-quotes.pdf",
        "Page\"double-quotes.pdf",
        "Page:colon.pdf",
        "Page@at-sign.pdf",
        "Page+plus-sign.pdf",
        "Page`backtick.pdf",
        "Page|pipe.pdf",
        "Page=equal-sign.pdf",
    ];

    let os_output_files = output_files.iter().map(OsStr::new).collect::<Vec<&OsStr>>();
    let os_output_files2 = output_files
        .iter()
        .map(|filename| curren_dir.join(filename))
        .collect::<Vec<PathBuf>>();

    let missing_files = WalkBuilder::new(&drawio_exporter.current_dir)
        .build()
        .filter_map(|r| r.ok())
        .filter(|d| {
            d.path().is_file()
                && match d.path().file_name() {
                    Some(filename) => os_output_files.contains(&filename),
                    None => false,
                }
        })
        .map(|d| d.into_path())
        .collect::<Vec<PathBuf>>();

    if !missing_files.is_empty() {
        return Err(anyhow!(format!("Missing files: {:#?}", missing_files)));
    }

    Ok(())
}

#[test]
fn export_file_using_shapes() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("shapes", true)?;

    let output = "+ export file : shapes/shapes.drawio
++ export page 1 : AWS
+++ generate pdf file
++ export page 2 : Azure
+++ generate pdf file
++ export page 3 : GCP
+++ generate pdf file
++ export page 4 : K8S
+++ generate pdf file";

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
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page-2
+++ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

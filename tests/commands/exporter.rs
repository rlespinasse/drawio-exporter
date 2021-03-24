use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

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

    let output = "+ export file : name_collision/name.drawio
++ export page 1 : Page-1
+++ generate pdf file
+ export file : name_collision/name-collision.drawio
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

    let output = "+ export file : tree/folder2/folder3/file3.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file
+ export file : tree/file1.drawio
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

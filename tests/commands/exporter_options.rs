use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_single_diagram_file_with_no_option_remove_page_suffix() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("single_page", true)?;

    let output = "+ export file : single_page/single-page.drawio
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
fn export_single_diagram_file_with_option_remove_page_suffix() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("single_page", true)?;

    let output = "+ export file : single_page/single-page.drawio
++ export page 1 : Page-1
+++ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--remove-page-suffix")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_using_specific_path() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : nominal.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir.join("types"))
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_using_unknown_path() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "Error: path must exists (as directory or file)";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir.join("unknown"))
        .assert()
        .failure()
        .stderr(contains(output));

    Ok(())
}

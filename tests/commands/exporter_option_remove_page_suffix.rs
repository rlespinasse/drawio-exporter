use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_single_diagram_file_with_no_option_remove_page_suffix() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("single_page", true)?;

    let output = "+ export file : single_page/single-page.drawio
- export page 1 : Page-1
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["single-page-Page-1.pdf"];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

#[test]
fn export_single_diagram_file_with_option_remove_page_suffix() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("single_page", true)?;

    let output = "+ export file : single_page/single-page.drawio
- export page 1 : Page-1
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--remove-page-suffix")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["single-page.pdf"];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

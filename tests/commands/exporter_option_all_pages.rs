use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_pdf_using_option_all_pages() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export all pages
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("pdf")
        .arg("--all-pages")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal.pdf"];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

#[test]
fn export_pdf_using_short_option_all_pages() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export all pages
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("pdf")
        .arg("-a")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal.pdf"];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

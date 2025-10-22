use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_xml_using_option_uncompressed() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export all pages
\\ generate xml file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("xml")
        .arg("--uncompressed")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal.xml"];

    utils::check_generate_files(&mut drawio_exporter, "xml", output_files)
}

#[test]
fn export_xml_using_short_option_uncompressed() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export all pages
\\ generate xml file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("xml")
        .arg("-u")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal.xml"];

    utils::check_generate_files(&mut drawio_exporter, "xml", output_files)
}

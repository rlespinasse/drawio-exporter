use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

fn export_xml_using_uncompressed_option(option: &str) -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export all pages
\\ generate xml file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("xml")
        .arg(option)
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal.xml"];

    utils::check_generate_files(&mut drawio_exporter, "xml", output_files)
}

#[test]
fn export_xml_using_option_uncompressed() -> Result<()> {
    export_xml_using_uncompressed_option("--uncompressed")
}

#[test]
fn export_xml_using_short_option_uncompressed() -> Result<()> {
    export_xml_using_uncompressed_option("-u")
}

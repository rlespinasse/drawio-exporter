use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_xml_using_option_uncompressed() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate xml file
- export page 2 : Page-2
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

    Ok(())
}

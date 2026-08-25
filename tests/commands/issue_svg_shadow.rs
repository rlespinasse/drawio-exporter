use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_svg_with_shadow() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("svg_shadow", true)?;

    let output = "+ export file : svg_shadow/svg_shadow.drawio
- export page 1 : Page-1
\\ generate svg file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("svg")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["svg_shadow-Page-1.svg"];

    utils::check_generate_files(&mut drawio_exporter, "svg", output_files)
}

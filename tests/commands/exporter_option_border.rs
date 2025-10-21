use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_using_option_border() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--border")
        .arg("10")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal-Page-1.pdf", "nominal-Page-2.pdf"];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_png_using_option_transparent() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate png file
- export page 2 : Page-2
\\ generate png file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("png")
        .arg("--transparent")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal-Page-1.png", "nominal-Page-2.png"];

    utils::check_generate_files(&mut drawio_exporter, "png", output_files)
}

#[test]
fn export_png_using_short_option_transparent() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate png file
- export page 2 : Page-2
\\ generate png file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("png")
        .arg("-t")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal-Page-1.png", "nominal-Page-2.png"];

    utils::check_generate_files(&mut drawio_exporter, "png", output_files)
}

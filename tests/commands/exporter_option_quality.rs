use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

fn export_jpg_using_quality_option(option: &str) -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate jpg file
- export page 2 : Page-2
\\ generate jpg file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("jpg")
        .arg(option)
        .arg("100")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal-Page-1.jpg", "nominal-Page-2.jpg"];

    utils::check_generate_files(&mut drawio_exporter, "jpg", output_files)
}

#[test]
fn export_jpg_using_option_quality() -> Result<()> {
    export_jpg_using_quality_option("--quality")
}

#[test]
fn export_jpg_using_short_option_quality() -> Result<()> {
    export_jpg_using_quality_option("-q")
}

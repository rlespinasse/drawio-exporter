use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_svg_using_option_svg_links_target_auto() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate svg file
- export page 2 : Page-2
\\ generate svg file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("svg")
        .arg("--svg-links-target")
        .arg("auto")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal-Page-1.svg", "nominal-Page-2.svg"];

    utils::check_generate_files(&mut drawio_exporter, "svg", output_files)
}

#[test]
fn export_svg_using_option_svg_links_target_new_win() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate svg file
- export page 2 : Page-2
\\ generate svg file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("svg")
        .arg("--svg-links-target")
        .arg("new-win")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal-Page-1.svg", "nominal-Page-2.svg"];

    utils::check_generate_files(&mut drawio_exporter, "svg", output_files)
}

#[test]
fn export_svg_using_option_svg_links_target_same_win() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate svg file
- export page 2 : Page-2
\\ generate svg file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("svg")
        .arg("--svg-links-target")
        .arg("same-win")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["nominal-Page-1.svg", "nominal-Page-2.svg"];

    utils::check_generate_files(&mut drawio_exporter, "svg", output_files)
}

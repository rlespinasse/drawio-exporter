use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

fn export_using_embed_diagram_option(format: &str, option: &str) -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = format!(
        "+ export file : types/nominal.drawio
- export page 1 : Page-1
\\ generate {} file
- export page 2 : Page-2
\\ generate {} file",
        format, format
    );

    drawio_exporter
        .cmd
        .arg("--format")
        .arg(format)
        .arg(option)
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = [format!("nominal-Page-1.{}", format),
        format!("nominal-Page-2.{}", format)];
    let output_files_refs: Vec<&str> = output_files.iter().map(|s| s.as_str()).collect();

    utils::check_generate_files(&mut drawio_exporter, format, output_files_refs)
}

#[test]
fn export_pdf_using_option_embed_diagram() -> Result<()> {
    export_using_embed_diagram_option("pdf", "--embed-diagram")
}

#[test]
fn export_png_using_option_embed_diagram() -> Result<()> {
    export_using_embed_diagram_option("png", "--embed-diagram")
}

#[test]
fn export_svg_using_option_embed_diagram() -> Result<()> {
    export_using_embed_diagram_option("svg", "--embed-diagram")
}

#[test]
fn export_pdf_using_short_option_embed_diagram() -> Result<()> {
    export_using_embed_diagram_option("pdf", "-e")
}

#[test]
fn export_png_using_short_option_embed_diagram() -> Result<()> {
    export_using_embed_diagram_option("png", "-e")
}

#[test]
fn export_svg_using_short_option_embed_diagram() -> Result<()> {
    export_using_embed_diagram_option("svg", "-e")
}

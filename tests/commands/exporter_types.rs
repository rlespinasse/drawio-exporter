use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_default_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_adoc_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate png file
+++ generate adoc file
+++ include links in adoc file
++ export page 2 : Page 2
+++ generate png file
+++ generate adoc file
+++ include links in adoc file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("adoc")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_md_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate png file
+++ generate md file
+++ include links in md file
++ export page 2 : Page 2
+++ generate png file
+++ generate md file
+++ include links in md file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("md")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_pdf_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("pdf")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_png_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate png file
++ export page 2 : Page 2
+++ generate png file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("png")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_xml_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate xml file
++ export page 2 : Page 2
+++ generate xml file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("xml")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_svg_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate svg file
++ export page 2 : Page 2
+++ generate svg file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("svg")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_vsdx_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : types/nominal.drawio
++ export page 1 : Page-1
+++ generate vsdx file
++ export page 2 : Page 2
+++ generate vsdx file";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("vsdx")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

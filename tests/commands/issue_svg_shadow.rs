use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_svg_with_shadow() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("svg_shadow", true)?;

    let output = "+ export file : svg_shadow/svg_shadow.drawio
- export page 1 : Page-1
\\ generate svg file";

    let output_err = format!(
        "Export failed: {}/svg_shadow/svg_shadow.drawio",
        &drawio_exporter.current_dir.display()
    );

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("svg")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .failure()
        .stdout(contains(output))
        .stderr(contains(output_err));

    Ok(())
}

use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_jpg_using_option_quality() -> Result<()> {
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
        .arg("--quality")
        .arg("100")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

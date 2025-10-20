use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;
use tempfile::tempdir;

#[test]
fn export_using_option_enable_plugins() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data(".", true)?;
    let tempdir = tempdir()?;

    let output = format!(
        "A {} B {} C {}",
        tempdir.path().display(),
        tempdir.path().display(),
        tempdir.path().display()
    );

    drawio_exporter
        .cmd
        .arg("--enable-plugins")
        .arg(tempdir.path())
        .assert()
        .success()
        .stderr(contains(output));

    Ok(())
}

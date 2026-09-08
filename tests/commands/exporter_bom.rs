use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

/// A regular compressed `<mxfile>` document prefixed with a UTF-8 BOM
/// (common from Windows editors/PowerShell redirects) must still be
/// recognized and exported like its non-BOM counterpart.
#[test]
fn export_compressed_mxfile_with_bom() -> Result<()> {
    let mut drawio_exporter =
        DrawioExporterCommand::new_using_data("mxfile_compressed_with_bom", true)?;

    let output = "+ export file : mxfile_compressed_with_bom/mxfile-compressed-with-bom.drawio
- export page 1 : Page-1
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

/// Same as above but for the uncompressed `<mxfile>` shape.
#[test]
fn export_uncompressed_mxfile_with_bom() -> Result<()> {
    let mut drawio_exporter =
        DrawioExporterCommand::new_using_data("mxfile_uncompressed_with_bom", true)?;

    let output = "+ export file : mxfile_uncompressed_with_bom/mxfile-uncompressed-with-bom.drawio
- export page 1 : Page-1
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

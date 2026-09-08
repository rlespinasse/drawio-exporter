use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

/// A regular compressed `<mxfile>` document prefixed with a UTF-8 BOM
/// (common from Windows editors/PowerShell redirects) must still be
/// recognized and exported like its non-BOM counterpart.
///
/// The BOM is added at test time rather than committed into the fixture:
/// a committed BOM'd file can be silently stripped by an editor or
/// formatter, which would leave this test green without it testing
/// anything.
#[test]
fn export_compressed_mxfile_with_bom() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("single_page", true)?;
    drawio_exporter.prepend_utf8_bom_if_missing("single_page/single-page.drawio")?;

    let output = "+ export file : single_page/single-page.drawio
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
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("mxfile_uncompressed", true)?;
    drawio_exporter.prepend_utf8_bom_if_missing("mxfile_uncompressed/mxfile-uncompressed.drawio")?;

    let output = "+ export file : mxfile_uncompressed/mxfile-uncompressed.drawio
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

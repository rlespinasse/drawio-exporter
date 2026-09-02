use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

/// draw.io itself reads and writes .drawio files saved as a bare
/// `<mxGraphModel>` document, with no enclosing `<mxfile><diagram>` wrapper.
/// Regression test for a bug where such a file silently produced zero output
/// (exit 0, no page/export lines, no error) because Mxfile parsing matched
/// no <diagram> child and nothing downstream reported the mismatch.
#[test]
fn export_file_without_mxfile_wrapper() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("bare_mx_graph_model", true)?;

    let output = "+ export file : bare_mx_graph_model/bare-mx-graph-model.drawio
- export page 1 : bare-mx-graph-model
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

/// Same bare <mxGraphModel> shape, but prefixed with a UTF-8 BOM (common
/// from Windows editors/PowerShell redirects). Must not be missed by the
/// root-tag detection just because of the leading BOM bytes.
#[test]
fn export_file_without_mxfile_wrapper_with_bom() -> Result<()> {
    let mut drawio_exporter =
        DrawioExporterCommand::new_using_data("bare_mx_graph_model_with_bom", true)?;

    let output = "+ export file : bare_mx_graph_model_with_bom/bare-mx-graph-model-with-bom.drawio
- export page 1 : bare-mx-graph-model-with-bom
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

/// Draw.io Desktop can save and re-read diagrams as `.drawio.svg`: a regular
/// SVG file with the mxfile XML embedded, HTML-entity-escaped, in a
/// `content="..."` attribute on the root element. Such files must be
/// recognized and exported like their plain `.drawio` counterparts.
#[test]
fn export_drawio_svg_file() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("drawio_svg", true)?;

    let output = "+ export file : drawio_svg/diagram.drawio.svg
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

/// The output filename for a `.drawio.svg` file must drop the whole
/// `.drawio.svg` suffix rather than just the trailing `.svg`, otherwise
/// `Path::file_stem` would leave a stray `.drawio` in generated filenames
/// (e.g. `diagram.drawio-Page-1.pdf` instead of `diagram-Page-1.pdf`).
#[test]
fn export_drawio_svg_file_uses_full_stem_for_output_filename() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("drawio_svg", true)?;

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success();

    let output_path = drawio_exporter
        .current_dir
        .join("drawio_svg/export/diagram-Page-1.pdf");
    assert!(output_path.exists(), "{:?} should exist", output_path);

    Ok(())
}

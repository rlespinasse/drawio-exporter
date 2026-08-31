use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_pdf_using_default_output_mode_is_relative() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("tree", true)?;

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success();

    let root = &drawio_exporter.current_dir;
    assert!(root.join("tree/export/file1-Page-1.pdf").exists());
    assert!(root.join("tree/folder1/export/file2.1-Page-1.pdf").exists());
    assert!(
        root.join("tree/folder2/folder3/export/file3-Page-1.pdf")
            .exists()
    );

    Ok(())
}

#[test]
fn export_pdf_using_option_output_mode_absolute_mirrors_source_tree() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("tree", true)?;

    let output = "+ export file : tree/file1.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file
+ export file : tree/folder1/file2.1.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file
+ export file : tree/folder1/file2.2.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file
+ export file : tree/folder2/folder3/file3.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--output-mode")
        .arg("absolute")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let root = &drawio_exporter.current_dir;
    assert!(root.join("export/tree/file1-Page-1.pdf").exists());
    assert!(root.join("export/tree/file1-Page-2.pdf").exists());
    assert!(root.join("export/tree/folder1/file2.1-Page-1.pdf").exists());
    assert!(root.join("export/tree/folder1/file2.2-Page-1.pdf").exists());
    assert!(
        root.join("export/tree/folder2/folder3/file3-Page-1.pdf")
            .exists()
    );

    // no file should have been exported next to the drawio files
    assert!(!root.join("tree/export").exists());
    assert!(!root.join("tree/folder1/export").exists());
    assert!(!root.join("tree/folder2/folder3/export").exists());

    Ok(())
}

#[test]
fn export_pdf_using_option_output_mode_absolute_avoids_name_collisions() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("output_mode_collision", true)?;

    drawio_exporter
        .cmd
        .arg("--output-mode")
        .arg("absolute")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success();

    let root = &drawio_exporter.current_dir;
    assert!(
        root.join("export/output_mode_collision/folder1/diagram-Page-1.pdf")
            .exists()
    );
    assert!(
        root.join("export/output_mode_collision/folder2/diagram-Page-1.pdf")
            .exists()
    );

    Ok(())
}

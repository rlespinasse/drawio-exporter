use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;
use std::path;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn export_using_option_git_ref_inside_simple_folder() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data(".", true)?;
    let tempdir = tempdir()?;

    let output = format!(
        "Error: can't explore path {}

Caused by:
    0: need to be a git repository {}
    1: could not find repository from '{}'; class=Repository (6); code=NotFound (-3)",
        tempdir.path().display(),
        tempdir.path().display(),
        tempdir.path().display()
    );

    drawio_exporter
        .cmd
        .arg("--on-changes")
        .arg("--git-ref")
        .arg("HEAD")
        .arg(tempdir.path())
        .assert()
        .failure()
        .stderr(contains(output));

    Ok(())
}

#[test]
fn export_using_option_git_ref_on_invalid_ref() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data(".", true)?;
    let output = format!(
        "Error: can't explore path {}

Caused by:
    0: can't found reference WRONG_REFERENCE on {}{}
    1: revspec 'WRONG_REFERENCE' not found; class=Reference (4); code=NotFound (-3)",
        &drawio_exporter.current_dir.display(),
        Path::new(".").canonicalize()?.join(".git").display(),
        path::MAIN_SEPARATOR
    );

    drawio_exporter
        .cmd
        .arg("--on-changes")
        .arg("--git-ref")
        .arg("WRONG_REFERENCE")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .failure()
        .stderr(contains(output));

    Ok(())
}

#[test]
fn export_using_option_git_ref_without_option_no_changes() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data(".", true)?;
    let output =
        "error: the following required arguments were not provided:\n  --on-changes\n\nUsage: drawio-exporter --on-changes --git-ref <reference> <PATH>\n\nFor more information, try \'--help\'.\n".to_string();

    drawio_exporter
        .cmd
        .arg("--git-ref")
        .arg("HEAD")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .failure()
        .stderr(contains(output));

    Ok(())
}

#[test]
fn export_using_option_git_ref_with_value_head() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data(".", true)?;

    drawio_exporter
        .cmd
        .arg("--on-changes")
        .arg("--git-ref")
        .arg("HEAD")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stderr("");

    Ok(())
}

#[test]
fn export_using_option_git_ref_with_value_root_commit_sha1() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("types", true)?;

    let output = "+ export file : nominal.drawio
++ export page 1 : Page-1
+++ generate pdf file
++ export page 2 : Page 2
+++ generate pdf file";

    drawio_exporter
        .cmd
        .arg("--on-changes")
        .arg("--git-ref")
        // short sha1 of the root commit
        .arg("cb9aec8")
        .arg("tests/data/types")
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

use crate::DrawioExporterCommand;
use crate::commands::utils;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_nothing() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("empty_folder", true)?;

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(""); // No Output

    Ok(())
}

#[test]
fn export_files_with_a_name_collision() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("name_collision", true)?;

    let output = "+ export file : name_collision/name-collision.drawio
- export page 1 : Page-1
\\ generate pdf file
+ export file : name_collision/name.drawio
- export page 1 : Page-1
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec![
        "name-collision-Page-1.pdf",
        //"name-Page-1.pdf",
    ];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

#[test]
fn export_files_from_a_folders_tree() -> Result<()> {
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
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec![
        "file1-Page-1.pdf",
        "file1-Page-2.pdf",
        "file2.1-Page-1.pdf",
        "file2.1-Page-2.pdf",
        "file2.2-Page-1.pdf",
        "file2.2-Page-2.pdf",
        "file3-Page-1.pdf",
        "file3-Page-2.pdf",
    ];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

#[test]
fn export_file_with_spaces() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("space", true)?;

    let output = "+ export file : space/file with spaces.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["file with spaces-Page-1.pdf", "file with spaces-Page-2.pdf"];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

#[test]
fn export_file_with_illegal_characters() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("illegal_characters", true)?;

    let output = "+ export file : illegal_characters/names.drawio
- export page 1 : Page-pound
\\ generate pdf file
- export page 2 : Page-percent
\\ generate pdf file
- export page 3 : Page-ampersand
\\ generate pdf file
- export page 4 : Page-left-curly-bracket
\\ generate pdf file
- export page 5 : Page-right-curly-bracket
\\ generate pdf file
- export page 6 : Page-back-slash
\\ generate pdf file
- export page 7 : Page-left-angle-bracket
\\ generate pdf file
- export page 8 : Page-right-angle-bracket
\\ generate pdf file
- export page 9 : Page-asterisk
\\ generate pdf file
- export page 10 : Page-question-mark
\\ generate pdf file
- export page 11 : Page-forward-slash
\\ generate pdf file
- export page 12 : Page-dollar-sign
\\ generate pdf file
- export page 13 : Page-exclamation-point
\\ generate pdf file
- export page 14 : Page-single-quotes
\\ generate pdf file
- export page 15 : Page-double-quotes
\\ generate pdf file
- export page 16 : Page-colon
\\ generate pdf file
- export page 17 : Page-at-sign
\\ generate pdf file
- export page 18 : Page-plus-sign
\\ generate pdf file
- export page 19 : Page-backtick
\\ generate pdf file
- export page 20 : Page-pipe
\\ generate pdf file
- export page 21 : Page-equal-sign
\\ generate pdf file
- export page 22 : Page-semicolon
\\ generate pdf file
- export page 23 : Page-comma
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec![
        "names-Page-pound.pdf",
        "names-Page-percent.pdf",
        "names-Page-ampersand.pdf",
        "names-Page-left-curly-bracket.pdf",
        "names-Page-right-curly-bracket.pdf",
        "names-Page-back-slash.pdf",
        "names-Page-left-angle-bracket.pdf",
        "names-Page-right-angle-bracket.pdf",
        "names-Page-asterisk.pdf",
        "names-Page-question-mark.pdf",
        "names-Page-forward-slash.pdf",
        "names-Page-dollar-sign.pdf",
        "names-Page-exclamation-point.pdf",
        "names-Page-single-quotes.pdf",
        "names-Page-double-quotes.pdf",
        "names-Page-colon.pdf",
        "names-Page-at-sign.pdf",
        "names-Page-plus-sign.pdf",
        "names-Page-backtick.pdf",
        "names-Page-pipe.pdf",
        "names-Page-equal-sign.pdf",
        "names-Page-semicolon.pdf",
        "names-Page-comma.pdf",
    ];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

#[test]
fn export_file_using_shapes() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("shapes", true)?;

    let output = "+ export file : shapes/shapes.drawio
- export page 1 : AWS
\\ generate pdf file
- export page 2 : Azure
\\ generate pdf file
- export page 3 : GCP
\\ generate pdf file
- export page 4 : K8S
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec![
        "shapes-AWS.pdf",
        "shapes-Azure.pdf",
        "shapes-GCP.pdf",
        "shapes-K8S.pdf",
    ];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

#[test]
fn export_file_without_any_diagram() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("empty_file", true)?;

    let output = "+ export file : empty_file/empty.drawio";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

#[test]
fn export_file_from_vscode() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("vscode", true)?;

    let output = "+ export file : vscode/vscode.drawio
- export page 1 : Page-1
\\ generate pdf file
- export page 2 : Page-2
\\ generate pdf file";

    drawio_exporter
        .cmd
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    let output_files = vec!["vscode-Page-1.pdf", "vscode-Page-2.pdf"];

    utils::check_generate_files(&mut drawio_exporter, "pdf", output_files)
}

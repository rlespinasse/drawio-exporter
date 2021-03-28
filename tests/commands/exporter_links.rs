use crate::DrawioExporterCommand;
use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;

#[test]
fn export_links_when_adoc_format() -> Result<()> {
    let mut drawio_exporter = DrawioExporterCommand::new_using_data("links", true)?;

    let output = "+ export file : links/links.drawio
++ export page 1 : Page-1
+++ generate png file
+++ generate adoc file
+++ include links in adoc file
warn: link not included, page link isn't supported, link 'Text Page Link 1' to data:page/id,ywT4ct3H2L5pf33UiNoI
warn: link not included, page link isn't supported, link 'Shape Page Link 1' to data:page/id,ywT4ct3H2L5pf33UiNoI
link 'Text Link 1' to https://github.com/rlespinasse/drawio-exporter
link 'Shape Link 1' to https://github.com/rlespinasse/drawio-exporter
++ export page 2 : Page-2
+++ generate png file
+++ generate adoc file
+++ include links in adoc file
warn: link not included, page link isn't supported, link 'Text Page Link 1' to data:page/id,Apcybv1_2TdogQw9BSl8
warn: link not included, page link isn't supported, link 'Shape Page Link 1' to data:page/id,Apcybv1_2TdogQw9BSl8
link 'Link on multiple Lines' to https://github.com/rlespinasse/drawio-exporter
++ export page 3 : empty-link
+++ generate png file
+++ generate adoc file
+++ include links in adoc file
warn: link not included, due to missing url: link 'Empty Link' to [missing]
++ export page 4 : empty-text
+++ generate png file
+++ generate adoc file
+++ include links in adoc file
warn: link not included, due to missing label: link '[missing]' to https://github.com/rlespinasse/drawio-exporter";

    drawio_exporter
        .cmd
        .arg("--format")
        .arg("adoc")
        .arg(&drawio_exporter.current_dir)
        .assert()
        .success()
        .stdout(contains(output));

    Ok(())
}

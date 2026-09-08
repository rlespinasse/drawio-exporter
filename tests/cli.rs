mod commands;

use std::process::Command;

use anyhow::Result;
use assert_cmd::cargo;
use fs_extra::{copy_items, dir};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, tempdir_in};

pub struct DrawioExporterCommand {
    pub cmd: Command,
    pub current_dir: PathBuf,
}

impl DrawioExporterCommand {
    pub fn new_file(&self, name: &str, content: &str) -> Result<()> {
        let mut file = File::create(self.current_dir.join(name))?;
        writeln!(file, "{}", content)?;
        Ok(())
    }

    pub fn new_file_in_folder(&self, folder: &str, name: &str, content: &str) -> Result<PathBuf> {
        let folder = self.current_dir.join(folder);
        fs::create_dir_all(folder.clone())?;
        let mut file = File::create(folder.join(name))?;
        writeln!(file, "{}", content)?;
        Ok(folder)
    }

    /// Prepends a UTF-8 BOM to an already-copied fixture file, in place,
    /// unless it already starts with one. Kept as a runtime transformation
    /// rather than a committed BOM'd fixture so an editor/formatter silently
    /// stripping the BOM from a checked-in file can't make a BOM regression
    /// test pass for the wrong reason.
    pub fn prepend_utf8_bom_if_missing(&self, relative_path: &str) -> Result<()> {
        let path = self.current_dir.join(relative_path);
        let content = fs::read(&path)?;

        if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
            return Ok(());
        }

        let mut with_bom = vec![0xEF, 0xBB, 0xBF];
        with_bom.extend(content);

        fs::write(&path, with_bom)?;
        Ok(())
    }

    pub fn new_cmd(&mut self) -> Result<()> {
        self.cmd = Command::new(cargo::cargo_bin!("drawio-exporter"));
        Ok(())
    }

    pub fn new_using_data(data: &str, use_local_folder: bool) -> Result<DrawioExporterCommand> {
        let cmd = Command::new(cargo::cargo_bin!("drawio-exporter"));

        let current_dir = match use_local_folder {
            true => {
                let tempdir_base = Path::new("target/debug/tmp");
                fs::create_dir_all(tempdir_base)?;

                let tempdir = tempdir_in(tempdir_base)?;
                tempdir.keep()
            }
            false => {
                let tempdir = tempdir()?;
                tempdir.keep()
            }
        };

        let options = dir::CopyOptions::new();
        copy_items(
            &[PathBuf::from("tests/data").join(data)],
            &current_dir,
            &options,
        )?;

        let wints_command = DrawioExporterCommand { cmd, current_dir };

        Ok(wints_command)
    }
}

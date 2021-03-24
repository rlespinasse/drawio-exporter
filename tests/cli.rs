mod commands;

use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
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

    pub fn new_cmd(&mut self) -> Result<()> {
        self.cmd = Command::cargo_bin("drawio-exporter")?;
        Ok(())
    }

    pub fn new_using_data(data: &str, use_local_folder: bool) -> Result<DrawioExporterCommand> {
        let cmd = Command::cargo_bin("drawio-exporter")?;

        let current_dir = match use_local_folder {
            true => {
                let tempdir_base = Path::new("target/debug/tmp");
                fs::create_dir_all(tempdir_base.clone())?;

                let tempdir = tempdir_in(tempdir_base)?;
                tempdir.into_path()
            }
            false => {
                let tempdir = tempdir()?;
                tempdir.into_path()
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

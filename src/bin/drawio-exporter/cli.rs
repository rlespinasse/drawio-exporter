use anyhow::Result;
use clap::Command;

use crate::commands;
use crate::commands::global_exec;

pub fn main() -> Result<()> {
    let matches = cli().get_matches();
    let command_exec = global_exec();
    command_exec(&matches)
}

fn cli() -> Command {
    let mut command = Command::new("drawio-exporter")
        .about("Command Line Client To Enhance Files Export Using Draw.io Application")
        .version(crate_version!())
        .long_version(crate_version!());

    for arg in commands::global_args() {
        command = command.arg(arg);
    }

    command
}

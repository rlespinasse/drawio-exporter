use anyhow::Result;
use clap::AppSettings;

use drawio_exporter::util::command_prelude::*;

use crate::commands;
use crate::commands::global_exec;

pub fn main() -> Result<()> {
    let matches = cli().get_matches();
    let command_exec = global_exec();
    command_exec(&matches)
}

fn cli() -> App {
    let args = commands::global_args();

    App::new("drawio-exporter")
        .about("Command Line Client To Enhance Files Export Using Draw.io Application")
        .version(crate_version!())
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::ColoredHelp,
        ])
        .args(args.as_ref())
}

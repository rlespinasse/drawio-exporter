use anyhow::Result;
use clap::ArgMatches;

use drawio_exporter::util::command_prelude::*;

mod exporter;

pub fn global_args() -> Vec<Arg> {
    exporter::args()
}

pub fn global_exec() -> fn(&ArgMatches<'_>) -> Result<()> {
    exporter::exec
}

use anyhow::Result;
use clap::{Arg, ArgMatches};

mod exporter;

pub fn global_args() -> Vec<Arg> {
    exporter::args()
}

pub fn global_exec() -> fn(&ArgMatches) -> Result<()> {
    exporter::exec
}

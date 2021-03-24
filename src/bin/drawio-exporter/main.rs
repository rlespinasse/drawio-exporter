#[macro_use]
extern crate clap;

use anyhow::Result;

mod cli;
mod commands;

fn main() -> Result<()> {
    cli::main()
}

mod cli;
mod commands;
mod config;
mod print_format;

use cli::Cli;
use env_logger;
use miette::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    env_logger::init();
    Cli::from_args().call()?;
    Ok(())
}

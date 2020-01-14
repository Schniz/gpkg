mod cli;
mod commands;
mod config;
mod print_format;

use cli::Cli;
use env_logger;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    let command = Cli::from_args();
    command.call();
}

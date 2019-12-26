mod config;
mod npm;
mod package_json;
mod install_package;
mod directory_portal;
mod cli;
mod commands;

use env_logger;
use structopt::StructOpt;
use cli::Cli;

fn main() {
    env_logger::init();
    let command = Cli::from_args();
    command.call();
}

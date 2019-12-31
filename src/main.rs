mod cli;
mod commands;
mod config;
mod directory_portal;
mod from;
mod install_package;
mod node_package_version;
mod npm;
mod package_json;
mod storage;

use cli::Cli;
use env_logger;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    let command = Cli::from_args();
    command.call();
}

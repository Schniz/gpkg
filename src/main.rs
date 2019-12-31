mod cli;
mod commands;
mod config;
mod directory_portal;
mod from;
mod install_package;
mod npm;
mod package_json;
mod storage;
mod node_package_version;

use cli::Cli;
use env_logger;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    let command = Cli::from_args();
    command.call();
}

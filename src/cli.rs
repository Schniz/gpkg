use crate::commands::{self, Command};
use crate::config::Config;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Commands {
    /// Install global binaries from npm
    #[structopt(name = "install")]
    Install(commands::Install),
}

impl Commands {
    pub fn call(self, config: Config) {
        match self {
            Self::Install(cmd) => cmd.call(config),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(flatten)]
    config: Config,
    #[structopt(subcommand)]
    subcommand: Commands,
}

impl Cli {
    pub fn call(self) {
        self.subcommand.call(self.config)
    }
}

use crate::commands::{self, Command};
use crate::config::Config;
use miette::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Commands {
    /// Install global binaries from npm
    #[structopt(name = "install", alias = "add", alias = "i")]
    Install(commands::Install),

    /// Generate completions for your shell
    #[structopt(name = "completions")]
    Completions(commands::Completions),

    /// List all installed binaries
    #[structopt(name = "list", alias = "ls")]
    List(commands::List),

    /// Uninstall a package
    #[structopt(name = "uninstall", alias = "remove", alias = "rm")]
    Uninstall(commands::Uninstall),
}

impl Commands {
    pub fn call(self, config: Config) -> Result<()> {
        match self {
            Self::Install(cmd) => cmd.apply(config)?,
            Self::Completions(cmd) => cmd.apply(config)?,
            Self::List(cmd) => cmd.apply(config)?,
            Self::Uninstall(cmd) => cmd.apply(config)?,
        };
        Ok(())
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
    pub fn call(self) -> Result<()> {
        self.subcommand.call(self.config)
    }
}

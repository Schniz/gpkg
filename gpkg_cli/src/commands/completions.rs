use super::Command;
use crate::cli::Cli;
use crate::config::Config;
use miette::Diagnostic;
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
pub struct Completions {
    shell: structopt::clap::Shell,
}

#[derive(Debug, Error, Diagnostic)]
pub enum Errors {}

impl Command for Completions {
    type Error = Errors;

    fn apply(self, _config: Config) -> Result<(), Self::Error> {
        let mut stdio = std::io::stdout();
        Cli::clap().gen_completions_to(env!("CARGO_PKG_NAME"), self.shell, &mut stdio);
        Ok(())
    }
}

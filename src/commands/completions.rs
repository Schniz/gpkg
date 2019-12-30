use structopt::StructOpt;
use super::Command;
use crate::config::Config;
use crate::cli::Cli;

#[derive(StructOpt, Debug)]
pub struct Completions {
    shell: structopt::clap::Shell,
}

impl Command for Completions {
    type Error = ();

    fn apply(self, _config: Config) -> Result<(), Self::Error> {
        let mut stdio = std::io::stdout();
        Cli::clap().gen_completions_to(env!("CARGO_PKG_NAME"), self.shell, &mut stdio);
        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
    }
}

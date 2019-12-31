use super::Command;
use crate::config::Config;
use crate::storage::Metadata;
use colored::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct List {}

impl Command for List {
    type Error = ();

    fn apply(self, config: Config) -> Result<(), Self::Error> {
        let binaries = Metadata::read_all(&config).expect("Can't read files");
        let max_width = binaries.iter().map(|x| x.binary_name.len()).max();

        if let Some(max_width) = max_width {
            for metadata in binaries.iter() {
                println!(
                    "{:<width$} {}",
                    metadata.binary_name,
                    format!(
                        "from {}, node {}",
                        metadata.package_name.cyan(),
                        metadata.node_version.yellow()
                    )
                    .dimmed()
                    .italic(),
                    width = max_width
                )
            }
        }

        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
    }
}

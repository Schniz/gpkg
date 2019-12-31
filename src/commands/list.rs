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
        let mut binaries = vec![];
        let metadata_entries = config
            .db_path()
            .read_dir()
            .expect("Can't access metadata path")
            .filter_map(Result::ok);

        for entry in metadata_entries {
            let s = std::fs::read(entry.path()).expect("Can't read file");
            let metadata: Metadata = serde_json::from_slice(&s).expect("Can't read JSON");
            let metadata = metadata.latest();
            binaries.push(metadata);
        }

        let max_width = binaries.iter().map(|x| x.binary_name.len()).max();
        binaries.sort_by(|a, b| a.binary_name.cmp(&b.binary_name));

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

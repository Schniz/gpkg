use super::Command;
use crate::config::Config;
use crate::print_format::PrintFormat;
use colored::*;
use gpkg::storage::{LatestMetadata, Metadata};
use prettytable::*;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
pub struct List {
    #[structopt(long, default_value = "list", possible_values = PrintFormat::variants())]
    format: PrintFormat,
}

#[derive(Debug, Error, miette::Diagnostic)]
pub enum Errors {
    #[error("Can't read files\n{cause}")]
    #[diagnostic()]
    CantReadFiles { cause: std::io::Error },
}

impl Command for List {
    type Error = Errors;

    fn apply(self, config: Config) -> Result<(), Self::Error> {
        let binaries = Metadata::read_all(config.bin_dir())
            .map_err(|cause| Errors::CantReadFiles { cause })?;
        match self.format {
            PrintFormat::List => print_metadata_pretty_list(&binaries),
            PrintFormat::Table => print_metadata_pretty_table(&binaries),
            PrintFormat::Json => print_metadata_json(&binaries),
        }
        Ok(())
    }
}

fn print_metadata_pretty_list(metadatas: impl AsRef<[LatestMetadata]>) {
    let max_width = metadatas
        .as_ref()
        .iter()
        .map(|x| x.binary_name.len())
        .max()
        .unwrap();
    for metadata in metadatas.as_ref().iter() {
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
        );
    }
}

fn print_metadata_pretty_table(metadatas: impl AsRef<[LatestMetadata]>) {
    use prettytable::*;

    let mut table = Table::new();

    table.add_row(row![Fc => "binary", "package", "node version"]);

    for metadata in metadatas.as_ref().iter() {
        table.add_row(row![
            metadata.binary_name,
            metadata.package_name,
            metadata.node_version,
        ]);
    }

    table.printstd();
}

fn print_metadata_json(metadatas: impl AsRef<[LatestMetadata]>) {
    let json = serde_json::to_string(metadatas.as_ref()).expect("Can't make into a JSON string");
    println!("{}", json);
}

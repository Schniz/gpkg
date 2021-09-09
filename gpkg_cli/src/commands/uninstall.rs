use super::Command;
use crate::config::Config;
use colored::*;
use gpkg::node_package_version::NodePackageVersion;
use gpkg::storage::Metadata;
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
pub struct Uninstall {
    version: NodePackageVersion,
}

#[derive(Debug, Error, miette::Diagnostic)]
pub enum Errors {
    #[error("Can't read metadata files")]
    #[diagnostic()]
    ReadingMetadata {
        #[source]
        source: std::io::Error,
    },

    #[error("Can't delete file {binary_path:?}")]
    #[diagnostic()]
    RemovingFile {
        binary_path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Can't delete directory {package_path:?}")]
    #[diagnostic()]
    RemovingDirectory {
        package_path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl Command for Uninstall {
    type Error = Errors;
    fn apply(self, config: Config) -> Result<(), Self::Error> {
        let binaries = Metadata::read_all(config.bin_dir())
            .map_err(|source| Errors::ReadingMetadata { source })?;
        let binaries = binaries
            .iter()
            .filter(|metadata| metadata.package_name == self.version.name());

        for binary_metadata in binaries {
            let binary_name = &binary_metadata.binary_name;
            let binary_path = config.bin_dir().join(binary_name);

            std::fs::remove_file(&binary_path).map_err(|source| Errors::RemovingFile {
                binary_path,
                source,
            })?;

            println!("Deleted binary {}", binary_metadata.binary_name.cyan());
        }

        let package_path = config.installations_dir().join(self.version.name());
        if package_path.exists() {
            std::fs::remove_dir_all(&package_path).map_err(|source| Errors::RemovingDirectory {
                package_path,
                source,
            })?;
            println!("Removed package {}", self.version.name().cyan());
        } else {
            println!(
                "Directory {} does not exist",
                package_path.to_str().unwrap().cyan()
            );
        }

        Ok(())
    }
}

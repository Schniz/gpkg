use super::Command;
use crate::config::Config;
use colored::*;
use gpkg::node_package_version::NodePackageVersion;
use gpkg::storage::Metadata;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Uninstall {
    version: NodePackageVersion,
}

impl Command for Uninstall {
    type Error = ();
    fn apply(self, config: Config) -> Result<(), Self::Error> {
        let binaries = Metadata::read_all(config.bin_dir()).expect("Can't read metadata files");
        let binaries = binaries
            .iter()
            .filter(|metadata| metadata.package_name == self.version.name());

        for binary_metadata in binaries {
            let binary_name = &binary_metadata.binary_name;
            let binary_path = config.bin_dir().join(binary_name);

            std::fs::remove_file(&binary_path)
                .expect(&format!("Can't delete file {:?}", binary_path));

            println!("Deleted binary {}", binary_metadata.binary_name.cyan());
        }

        let package_path = config.installations_dir().join(self.version.name());
        if package_path.exists() {
            std::fs::remove_dir_all(&package_path)
                .expect(&format!("Can't remove directory {:?}", package_path));
            println!("Removed package {}", self.version.name().cyan());
        } else {
            println!(
                "Directory {} does not exist",
                package_path.to_str().unwrap().cyan()
            );
        }

        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
        unimplemented!();
    }
}

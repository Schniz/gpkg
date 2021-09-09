use crate::binary::Binary;
use crate::directory_portal::DirectoryPortal;
use crate::node_package_version::NodePackageVersion;
use crate::npm;
use crate::package_json::{PackageEngines, PackageRoot};
use crate::storage::{LatestMetadata, Metadata};
use log::*;
use miette::{Diagnostic, NamedSource, SourceOffset, SourceSpan};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct InstalledPackage {
    name: String,
    bin: PackageBinary,
}

impl InstalledPackage {
    fn binaries(self) -> HashMap<String, String> {
        match self.bin {
            PackageBinary::Single(state) => {
                let mut bin = HashMap::new();
                bin.insert(self.name, state);
                bin
            }
            PackageBinary::Multiple(state) => state,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum PackageBinary {
    Single(String),
    Multiple(HashMap<String, String>),
}

fn package_metadata_for_requested_package(
    dependency: &str,
    version: &str,
    current_node_version: impl Into<String>,
) -> PackageRoot {
    PackageRoot {
        name: format!(
            "{}_global_installation",
            dependency.to_string().replace("/", "__").replace("@", "")
        ),
        dependencies: {
            let mut deps = HashMap::default();
            deps.insert(dependency.into(), version.to_string());
            deps
        },
        engines: PackageEngines {
            node: current_node_version.into(),
        },
    }
}

fn infer_current_node_version() -> std::io::Result<String> {
    let cmd = Command::new("node")
        .arg("--version")
        .stdout(Stdio::piped())
        .output()?;
    let version = std::str::from_utf8(&cmd.stdout)
        .ok()
        .ok_or(std::io::ErrorKind::UnexpectedEof)?
        .trim()
        .to_string();
    Ok(version)
}

#[derive(Debug, Error, Diagnostic)]
pub enum Errors {
    #[error(transparent)]
    #[diagnostic()]
    IoError(#[from] std::io::Error),
    #[error("Can't parse package.json")]
    #[diagnostic(help("Please check what's going on in the package!"))]
    SerdeError {
        #[source]
        error: serde_json::Error,
        json: NamedSource,
        #[snippet(json, message = "package.json")]
        snip: SourceSpan,
        #[highlight(snip, label = "here")]
        highlight: SourceSpan,
    },
    #[diagnostic()]
    #[error("Package {0:?} is already installed")]
    PackageAlreadyInstalled(String),
}

pub fn install_package<InstallationDir: AsRef<Path>, BinDir: AsRef<Path>>(
    requested_package: &NodePackageVersion,
    installation_dir: InstallationDir,
    bin_dir: BinDir,
) -> Result<(), Errors> {
    let node_version = infer_current_node_version()?;
    debug!("Current node version: {}", node_version);
    let node_binary_path = get_node_binary_location();
    debug!(
        "Current node binary path: {}",
        node_binary_path.as_path().display()
    );
    let package = package_metadata_for_requested_package(
        requested_package.name(),
        requested_package.version(),
        &node_version,
    );
    let package_json_contents = serde_json::to_string_pretty(&package).unwrap();
    let target_path = installation_dir
        .as_ref()
        .join(requested_package.name().replace("/", "__"));
    if target_path.exists() {
        return Err(Errors::PackageAlreadyInstalled(
            requested_package.name().to_string(),
        ));
    }
    let portal = DirectoryPortal::new_in(std::env::temp_dir(), &target_path);
    std::fs::write(portal.join("package.json"), package_json_contents)
        .expect("Can't write package.json file");

    npm::install(&portal).expect("Can't access status code");

    let installed_package_json_path = portal
        .join("node_modules")
        .join(requested_package.name())
        .join("package.json");
    let installed_package_json = std::fs::read_to_string(&installed_package_json_path)
        .expect("Can't open package.json file");
    let installed_package: InstalledPackage = serde_json::from_str(&installed_package_json)
        .map_err(|error| {
            let len = installed_package_json.len();
            let x =
                SourceOffset::from_location(&installed_package_json, error.line(), error.column());
            Errors::SerdeError {
                json: NamedSource::new(
                    requested_package.name().to_string(),
                    installed_package_json,
                ),
                snip: (0, len).into(),
                highlight: (x.offset(), 1).into(),
                error,
            }
        })?;

    let teleport_path = portal.teleport()?;

    for binary_name in installed_package.binaries().keys() {
        let metadata = Metadata::V1(LatestMetadata {
            binary_name: binary_name.to_string(),
            package_name: requested_package.name().to_string(),
            node_version: node_version.to_string(),
        });
        let target_binary_path = teleport_path
            .join("node_modules")
            .join(".bin")
            .join(binary_name);
        let script_path = bin_dir.as_ref().join(binary_name);
        let binary = Binary::new(
            metadata,
            &script_path,
            &target_binary_path,
            &node_binary_path,
        );
        binary.create_script().expect("Can't create script");
    }

    Ok(())
}

// Still not sure whether to add `fnm exec {version} {node args}`
// command in fnm, or to keep this hard Node binary string here
fn get_node_binary_location() -> std::path::PathBuf {
    let stdout = Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg("node")
        .output()
        .expect("Can't read output from 'which' command")
        .stdout;
    let location = std::str::from_utf8(&stdout)
        .expect("Can't decode result from which command")
        .split('\n')
        .next()
        .expect("Got an empty result")
        .trim();
    debug!("`which node` returned location {:?}", &location);
    let node_path = std::fs::canonicalize(location).expect("Can't canonicalize node path");
    node_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test() {
        env_logger::builder().is_test(true).init();

        let installation_dir = tempdir::TempDir::new("installations").unwrap();
        let bin_dir = tempdir::TempDir::new("bin").unwrap();
        let package = NodePackageVersion::from_str("qnm@1.0.1").unwrap();

        install_package(&package, installation_dir.path(), bin_dir.path())
            .expect("Can't install qnm");

        let only_child = bin_dir
            .path()
            .read_dir()
            .expect("Can't read temp dir")
            .next()
            .expect("No files in temp dir")
            .expect("Can't access bin file");
        let stdout = Command::new(only_child.path())
            .arg("--version")
            .output()
            .expect("Can't read output from command")
            .stdout;
        let version = std::str::from_utf8(&stdout)
            .expect("Can't decode output")
            .trim();
        assert_eq!(version, "1.0.1");
    }
}

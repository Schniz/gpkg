use crate::from;
use crate::node_package_version::NodePackageVersion;
use crate::package_json::{PackageEngines, PackageRoot};
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize)]
pub struct InstalledPackage {
    bin: HashMap<String, String>,
}

fn package_metadata_for_requested_package(
    dependency: &str,
    version: &str,
    current_node_version: impl Into<String>,
) -> PackageRoot {
    PackageRoot {
        name: format!("{}_global_installation", dependency.to_string()),
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

use crate::config::Config;
use crate::directory_portal::DirectoryPortal;
use crate::npm;
use crate::storage::{LatestMetadata, Metadata};

#[derive(Debug)]
pub enum Errors {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    PackageAlreadyInstalled,
}

from!(Errors, {
    std::io::Error => IoError,
    serde_json::Error => SerdeError
});

pub fn install_package(
    requested_package: &NodePackageVersion,
    config: &Config,
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
    let target_path = config.installations_dir().join(requested_package.name());
    if target_path.exists() {
        return Err(Errors::PackageAlreadyInstalled);
    }
    let portal = DirectoryPortal::new(&target_path);
    std::fs::write(portal.join("package.json"), package_json_contents)
        .expect("Can't write package.json file");

    npm::install(&portal).expect("Can't access status code");

    let installed_package_json_path = portal
        .join("node_modules")
        .join(requested_package.name())
        .join("package.json");
    let installed_package_json = std::fs::read_to_string(&installed_package_json_path)
        .expect("Can't open package.json file");
    let installed_package: InstalledPackage = serde_json::from_str(&installed_package_json)?;

    let teleport_path = portal.teleport()?;

    for binary_name in installed_package.bin.keys() {
        let metadata = Metadata::V1(LatestMetadata {
            binary_name: binary_name.to_string(),
            package_name: requested_package.name().to_string(),
            node_version: node_version.to_string(),
        });
        let target_binary_path = teleport_path
            .join("node_modules")
            .join(".bin")
            .join(binary_name);
        let script_path = config.bin_dir().join(binary_name);
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn test() {
        env_logger::builder().is_test(true).init();
        let config = Config::default();
        let package = NodePackageVersion::from_str("qnm@1.0.1").unwrap();
        install_package(&package, &config).expect("Can't install qnm");
        let only_child = config
            .bin_dir()
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

use std::path::Path;

struct Binary<P1: AsRef<Path>, P2: AsRef<Path>, NodePath: AsRef<Path>> {
    metadata: Metadata,
    symlink_path: P1,
    target_path: P2,
    node_binary_path: NodePath,
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

impl<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>> Binary<P1, P2, P3> {
    pub fn new(metadata: Metadata, symlink_path: P1, target_path: P2, node_path: P3) -> Self {
        Self {
            metadata,
            symlink_path,
            target_path,
            node_binary_path: node_path,
        }
    }

    pub fn script_src(&self) -> String {
        let binary_path = self
            .node_binary_path
            .as_ref()
            .parent()
            .expect("Got node with no parent");
        let source = format!(
            r#"
                #!/bin/sh
                # metadata: {metadata_json}
                export PATH={node_binary_path:?}:$PATH
                {binary_path:?} "$@"
            "#,
            metadata_json = base64::encode(&serde_json::to_string(&self.metadata).unwrap()),
            binary_path = self.target_path.as_ref(),
            node_binary_path = binary_path,
        );
        unindent::unindent(&source)
    }

    pub fn create_script(self) -> std::io::Result<P1> {
        let src = self.script_src();
        std::fs::write(&self.symlink_path, &src)?;
        set_permissions(&self.symlink_path)?;

        Ok(self.symlink_path)
    }
}

#[cfg(unix)]
fn set_permissions(script_path: impl AsRef<Path>) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = script_path.as_ref().metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o744); // set executable
    std::fs::set_permissions(&script_path, permissions)?;
    Ok(())
}

#[cfg(windows)]
fn set_permissions(_script_path: impl AsRef<Path>) -> std::io::Result<()> {
    Ok(())
}

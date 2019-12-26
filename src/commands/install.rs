use super::Command;
use crate::config::Config;
use crate::install_package;
use structopt::StructOpt;
use log::*;

#[derive(StructOpt, Debug)]
pub struct Install {
    /// The npm package to install. Takes the format of `name[@version]`
    package: InstallablePackage,
}

#[derive(Debug)]
struct InstallablePackage {
    name: String,
    version: Option<String>,
}

impl std::str::FromStr for InstallablePackage {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split("@");
        match (parts.next().map(str::trim), parts.next().map(str::trim)) {
            (Some(name), None) | (Some(name), Some("")) => Ok(Self {
                name: name.into(),
                version: None,
            }),
            (Some(name), Some(version)) => Ok(Self {
                name: name.into(),
                version: Some(version.into()),
            }),
            _ => Err(format!(
                "Can't parse version {}. The format is {:?}",
                &s, "name[@version]"
            )),
        }
    }
}

impl Command for Install {
    type Error = install_package::Errors;

    fn apply(self, config: Config) -> Result<(), Self::Error> {
        debug!("Installing package {:?}", &self.package);
        install_package::install_package(&self.package.name, self.package.version, &config)?;
        Ok(())
    }

    fn handle_error(err: Self::Error) {
        dbg!(err);
        unimplemented!();
    }
}

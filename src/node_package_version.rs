use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct NodePackageVersion {
    name: String,
    version: Option<String>,
}

impl NodePackageVersion {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        self.version.as_ref().map(|x| &x[..]).unwrap_or("latest")
    }
}

impl FromStr for NodePackageVersion {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_version() {
        let version = NodePackageVersion::from_str("qnm").unwrap();
        assert_eq!(
            version,
            NodePackageVersion {
                name: "qnm".into(),
                version: None
            }
        );
    }

    #[test]
    fn version_with_tag() {
        let version = NodePackageVersion::from_str("qnm@next").unwrap();
        assert_eq!(
            version,
            NodePackageVersion {
                name: "qnm".into(),
                version: Some("next".into())
            }
        );
    }

    #[test]
    fn with_spaces_in_version() {
        let version = NodePackageVersion::from_str(" qnm @ 1.0.1 ").unwrap();
        assert_eq!(
            version,
            NodePackageVersion {
                name: "qnm".into(),
                version: Some("1.0.1".into())
            }
        );
    }
}

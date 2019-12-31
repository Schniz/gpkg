use crate::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataV1 {
    pub binary_name: String,
    pub package_name: String,
    pub node_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "metadata_version", rename_all = "lowercase")]
pub enum Metadata {
    V1(MetadataV1),
}

pub type LatestMetadata = MetadataV1;

impl Metadata {
    pub fn latest(self) -> LatestMetadata {
        match self {
            Self::V1(m) => m,
        }
    }

    pub fn path_for(binary_name: &str, config: &Config) -> std::path::PathBuf {
        config.db_path().join(binary_name).with_extension("json")
    }

    pub fn read_all(config: &Config) -> std::io::Result<Vec<LatestMetadata>> {
        let mut binaries = vec![];
        let metadata_entries = config.db_path().read_dir()?.filter_map(Result::ok);

        for entry in metadata_entries {
            let s = std::fs::read(entry.path())?;
            let metadata: Metadata = serde_json::from_slice(&s)
                .expect(&format!("Malformed JSON file at {:?}", entry.path()));
            let metadata = metadata.latest();
            binaries.push(metadata);
        }

        binaries.sort_by(|a, b| a.binary_name.cmp(&b.binary_name));

        Ok(binaries)
    }
}

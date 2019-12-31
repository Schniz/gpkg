use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetadataV1 {
    pub binary_name: String,
    pub package_name: String,
    pub node_version: String,
}

#[derive(Serialize, Deserialize)]
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
}

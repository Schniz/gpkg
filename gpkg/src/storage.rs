use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::path::Path;

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

    pub fn read_all<BinDir: AsRef<Path>>(bin_dir: BinDir) -> std::io::Result<Vec<LatestMetadata>> {
        let mut binaries = vec![];
        let metadata_entries = bin_dir.as_ref().read_dir()?.filter_map(Result::ok);

        for entry in metadata_entries {
            let metadata = Metadata::try_from(std::fs::File::open(entry.path())?)?.latest();
            binaries.push(metadata);
        }

        binaries.sort_by(|a, b| a.binary_name.cmp(&b.binary_name));

        Ok(binaries)
    }
}

impl std::convert::TryFrom<std::fs::File> for Metadata {
    type Error = std::io::Error;

    fn try_from(file: std::fs::File) -> Result<Self, Self::Error> {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("# metadata: ") {
                let metadata: Metadata = serde_json::from_slice(
                    &base64::decode(line.trim_start_matches("# metadata: "))
                        .ok()
                        .ok_or(std::io::ErrorKind::UnexpectedEof)?,
                )
                .ok()
                .ok_or(std::io::ErrorKind::UnexpectedEof)?;
                return Ok(metadata);
            }
        }

        Err(std::io::ErrorKind::UnexpectedEof.into())
    }
}

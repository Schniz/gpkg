use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// Gpkg root directory
    #[structopt(env = "GPKG_ROOT_DIR", name = "root-dir")]
    root_dir: Option<PathBuf>,
}

impl Config {
    pub fn root_dir(&self) -> PathBuf {
        self.root_dir.clone().unwrap_or_else(|| {
            let home_dir = dirs::home_dir().expect("Can't reach home dir");
            home_dir.join(".gpkg")
        })
    }

    pub fn installations_dir(&self) -> PathBuf {
        let path = self.root_dir().join("installations");
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Can't create missing installations dir");
        }
        path
    }

    pub fn bin_dir(&self) -> PathBuf {
        let path = self.root_dir().join("bin");
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Can't create installations dir");
        }
        path
    }

    pub fn db_path(&self) -> PathBuf {
        let path = self.root_dir().join("metadata");
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Can't create installations dir");
        }
        path
    }
}

#[cfg(test)]
impl Default for Config {
    fn default() -> Self {
        use tempdir::TempDir;

        Self {
            root_dir: Some(
                TempDir::new("tests")
                    .expect("Can't create temp dir")
                    .into_path(),
            ),
        }
    }
}

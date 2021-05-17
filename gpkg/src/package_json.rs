use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageRoot {
    pub name: String,
    pub dependencies: HashMap<String, String>,
    pub engines: PackageEngines,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageEngines {
    pub node: String,
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PackageRoot {
    pub name: String,
    pub dependencies: HashMap<String, String>,
    pub engines: PackageEngines,
}

#[derive(Serialize, Deserialize)]
pub struct PackageEngines {
    pub node: String,
}

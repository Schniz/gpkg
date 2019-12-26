use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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

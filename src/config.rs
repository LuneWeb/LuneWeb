use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LunewebConfigDev {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LunewebConfig {
    pub dev: LunewebConfigDev,
}

impl From<PathBuf> for LunewebConfig {
    fn from(value: PathBuf) -> Self {
        let path = value.join(PathBuf::from("luneweb.toml"));
        let bytes_content = fs::read(path).expect("luneweb.toml doesn't exist");
        let content = String::from_utf8(bytes_content).unwrap();
        toml::from_str(&content).unwrap()
    }
}

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LunewebConfigDev {
    pub url: Option<String>,
    pub pkg_manager: Option<String>,
    pub pkg_install: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LunewebConfigApp {
    pub luau: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LunewebConfig {
    pub dev: Option<LunewebConfigDev>,
    pub app: Option<LunewebConfigApp>,
}

impl From<PathBuf> for LunewebConfig {
    fn from(value: PathBuf) -> Self {
        let path = value.join(PathBuf::from("luneweb.toml"));
        let bytes_content =
            fs::read(&path).unwrap_or_else(|_| panic!("luneweb.toml doesn't exist at '{path:?}'"));
        let content = String::from_utf8(bytes_content).unwrap();
        toml::from_str(&content).unwrap_or(Self {
            dev: None,
            app: None,
        })
    }
}

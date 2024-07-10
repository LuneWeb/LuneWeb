use std::fs;
use std::path::PathBuf;

use luneweb_app::config::AppConfig;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LunewebConfigDev {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LunewebConfigApp {
    pub name: Option<String>,
    pub luau: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LunewebConfig {
    pub dev: Option<LunewebConfigDev>,
    pub app: LunewebConfigApp,
}

impl From<LunewebConfig> for AppConfig {
    fn from(val: LunewebConfig) -> Self {
        AppConfig {
            url: val
                .dev
                .and_then(|dev| dev.url)
                .unwrap_or("http://localhost:5173/".into()),
            window_title: val.app.name.unwrap_or("LuneWeb".into()),
        }
    }
}

impl From<PathBuf> for LunewebConfig {
    fn from(value: PathBuf) -> Self {
        let path = value.join(PathBuf::from("luneweb.toml"));
        let bytes_content =
            fs::read(&path).unwrap_or_else(|_| panic!("luneweb.toml doesn't exist at '{path:?}'"));
        let content = String::from_utf8(bytes_content).unwrap();
        toml::from_str(&content).unwrap_or_else(|err| panic!("Failed to parse luneweb.toml\n{err}"))
    }
}

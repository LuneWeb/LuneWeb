use std::{env::current_dir, path::PathBuf};

use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use tokio::fs;

use crate::VERSION;

pub const TYPE_DEFS: &str = include_str!("../../../globals.d.luau");
pub const LIBRARIES: Dir<'static> = include_dir!("libraries");

pub static HOME_PATH: Lazy<PathBuf> = Lazy::new(|| {
    directories::BaseDirs::new()
        .unwrap()
        .home_dir()
        .join(".luneweb")
});

pub static PROJECT_PATH: Lazy<PathBuf> = Lazy::new(|| current_dir().unwrap().join(".luneweb"));

pub async fn create_home() -> Result<(), std::io::Error> {
    fs::create_dir_all(&*HOME_PATH).await
}

pub async fn install_typedefs() -> Result<(), std::io::Error> {
    let path = HOME_PATH.join(format!(".type_defs-{VERSION}.d.luau"));

    if path.exists() {
        fs::remove_file(&path).await?;
    }

    fs::write(&path, TYPE_DEFS).await?;

    println!("Installed luau type definitions file at {path:?}");

    Ok(())
}

pub async fn install_libraries() -> Result<(), std::io::Error> {
    let dir = PROJECT_PATH.join(format!(".libraries-{VERSION}"));

    if dir.exists() {
        fs::remove_dir_all(&dir).await?;
    }

    fs::create_dir_all(&dir).await?;

    LIBRARIES.extract(&dir)?;

    println!("Installed luau libraries at {dir:?}");

    Ok(())
}

pub async fn run() -> Result<(), mlua::Error> {
    create_home().await?;
    install_typedefs().await?;
    install_libraries().await?;

    Ok(())
}

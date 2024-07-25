use std::path::PathBuf;

use once_cell::sync::Lazy;
use tokio::fs;

use crate::VERSION;

pub const TYPE_DEFS: &str = include_str!("../../../globals.d.luau");

pub static HOME_PATH: Lazy<PathBuf> = Lazy::new(|| {
    directories::BaseDirs::new()
        .unwrap()
        .home_dir()
        .join(".luneweb")
});

pub async fn create_home() -> Result<(), std::io::Error> {
    fs::create_dir_all(&*HOME_PATH).await
}

pub async fn install_typedefs() -> Result<(), std::io::Error> {
    let path = HOME_PATH.join(format!(".type_defs-{VERSION}.d.luau"));

    if path.exists() {
        println!("Deleting existing type definitions for this version");
        fs::remove_file(&path).await?;
    }

    println!("Installing type definition files");
    fs::write(&path, TYPE_DEFS).await?;

    println!("Installed type definition files at {path:?}");

    Ok(())
}

pub async fn run() -> Result<(), mlua::Error> {
    create_home().await?;
    install_typedefs().await?;

    Ok(())
}

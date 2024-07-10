use std::{fs, path::PathBuf};

use directories::BaseDirs;
use once_cell::sync::Lazy;

use crate::{LUAU_TYPES, VERSION};

static HOME_DIR: Lazy<PathBuf> = Lazy::new(|| {
    BaseDirs::new()
        .expect("Failed to get base directories")
        .home_dir()
        .to_path_buf()
});

pub fn install_types() -> Result<(), String> {
    let directory = HOME_DIR.join(".luneweb");
    let file_directory = directory.join(format!(".type_defs-{VERSION}.d.luau"));

    if !directory.is_dir() {
        fs::create_dir_all(&directory).unwrap();
    }

    fs::write(&file_directory, LUAU_TYPES).unwrap();

    println!("Installed Luau type definition files at: {file_directory:?}");

    Ok(())
}

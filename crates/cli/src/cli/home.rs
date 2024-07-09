use std::{fs, path::PathBuf};

use crate::{LUAU_TYPES, VERSION};

pub fn luneweb_home() -> Result<PathBuf, String> {
    let home = match homedir::my_home() {
        Ok(dir) => dir,
        Err(err) => return Err(err.to_string()),
    };

    match home {
        Some(dir) => Ok(dir),
        None => Err("Failed to find home directory".to_string()),
    }
}

pub fn install_types() -> Result<(), String> {
    let home = luneweb_home().unwrap();
    let directory = home.join(".luneweb");

    if !directory.is_dir() {
        fs::create_dir_all(&directory).unwrap();
    }

    fs::write(
        directory.join(format!(".type_defs-{VERSION}.d.luau")),
        LUAU_TYPES,
    )
    .unwrap();

    println!("Installed Luau type definition files at: '{directory:?}'");

    Ok(())
}

use std::{fs, path::PathBuf};

use include_dir::{include_dir, Dir};

use super::set_cwd;

const LUAU_TYPES: Dir = include_dir!("types/");

pub fn setup(dir: Option<PathBuf>) {
    let cwd = set_cwd(dir);
    let directory = cwd.join("types");

    if directory.is_dir() {
        fs::remove_dir_all(&directory).unwrap();
    }

    fs::create_dir_all(&directory).unwrap();

    for file in LUAU_TYPES.files() {
        fs::write(directory.join(file.path()), file.contents()).unwrap();
    }
}

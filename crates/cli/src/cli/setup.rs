use std::path::PathBuf;

use super::home::install_types;

pub fn setup(_dir: Option<PathBuf>) {
    // TODO: add the location of type definition files to vscode settings automatically

    install_types().unwrap();
}

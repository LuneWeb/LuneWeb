use std::path::PathBuf;

pub fn clean_path(path: PathBuf) -> PathBuf {
    path_clean::clean(path)
}

pub fn strip_alias(path: PathBuf) -> mlua::Result<Option<(String, String)>> {
    if let Some(path) = path.to_string_lossy().strip_prefix("@") {
        path.split_once("/").map_or_else(
            || Err(mlua::Error::runtime("Require path is missing '/'")),
            |x| Ok(Some((x.0.to_string(), x.1.to_string()))),
        )
    } else {
        Ok(None)
    }
}

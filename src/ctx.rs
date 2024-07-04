use crate::LuneWebError;
use include_dir::Dir;

#[derive(Default)]
pub struct Context<'a> {
    luau: Option<Dir<'a>>,
    javascript: Option<Dir<'a>>,
}

impl<'ctx> Context<'ctx> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn luau_dir(mut self, dir: Dir<'ctx>) -> Result<Self, LuneWebError> {
        if dir.contains("init.luau") {
            self.luau = Some(dir);
            Ok(self)
        } else {
            Err("The provided Luau directory does not contain 'init.luau'"
                .to_string()
                .into())
        }
    }

    pub fn javascript_dir(mut self, dir: Dir<'ctx>) -> Result<Self, LuneWebError> {
        if dir.contains("index.js") {
            self.javascript = Some(dir);
            Ok(self)
        } else {
            Err(
                "The provided Javascript directory does not contain 'index.js'"
                    .to_string()
                    .into(),
            )
        }
    }
}

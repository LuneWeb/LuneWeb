use std::env::current_dir;

use crate::LuneWebError;
use include_dir::Dir;
use lune_std::context::GlobalsContextBuilder;

#[derive(Default)]
pub struct Context {
    pub(crate) luau: Option<Dir<'static>>,
    pub(crate) javascript: Option<Dir<'static>>,
    pub(crate) lune_ctx: GlobalsContextBuilder,
}

impl Context {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Make sure to use this before calling `luau_dir` or you'll remove all the bundled modulescripts
    pub fn luau_ctx(mut self, ctx: GlobalsContextBuilder) -> Self {
        self.lune_ctx = ctx;
        self
    }

    pub fn luau_dir(mut self, dir: Dir<'static>) -> Result<Self, LuneWebError> {
        if dir.contains("init.luau") {
            let cwd = current_dir().expect("Failed to get the current working directory");

            for file in dir.files() {
                self.lune_ctx
                    .with_script(cwd.join(file.path()), file.contents().into())
            }

            self.luau = Some(dir);

            Ok(self)
        } else {
            Err("The provided Luau directory does not contain 'init.luau'"
                .to_string()
                .into())
        }
    }

    pub fn javascript_dir(mut self, dir: Dir<'static>) -> Self {
        self.javascript = Some(dir);
        self
    }
}

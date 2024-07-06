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

    pub fn luau_ctx(mut self, ctx: GlobalsContextBuilder) -> Self {
        self.lune_ctx = ctx;
        self
    }

    pub fn javascript_dir(mut self, dir: Dir<'static>) -> Self {
        self.javascript = Some(dir);
        self
    }
}

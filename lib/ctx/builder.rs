use lune_std::context::GlobalsContextBuilder;
use std::path::PathBuf;

#[derive(Default)]
pub struct ContextBuilder {
    pub(crate) lune_ctx_builder: Option<GlobalsContextBuilder>,
    pub(crate) luau_input: Option<PathBuf>,
}

impl ContextBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_lune_ctx(mut self, ctx_builder: GlobalsContextBuilder) -> Self {
        self.lune_ctx_builder = Some(ctx_builder);
        self
    }

    pub fn with_luau_input<T: Into<PathBuf>>(mut self, input: T) -> Self {
        self.luau_input = Some(input.into());
        self
    }
}

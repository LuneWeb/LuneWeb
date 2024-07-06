use lune_std::context::GlobalsContextBuilder;

#[derive(Default)]
pub struct ContextBuilder {
    pub(crate) lune_ctx_builder: Option<GlobalsContextBuilder>,
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
}

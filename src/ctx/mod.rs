pub use builder::ContextBuilder;
use lune_std::context::GlobalsContext;

mod builder;

pub(crate) struct Context {
    pub(crate) lune_ctx: GlobalsContext,
}

impl Default for Context {
    fn default() -> Self {
        ContextBuilder::default().into()
    }
}

impl From<ContextBuilder> for Context {
    fn from(value: ContextBuilder) -> Self {
        Self {
            lune_ctx: value.lune_ctx_builder.unwrap_or_default().into(),
        }
    }
}

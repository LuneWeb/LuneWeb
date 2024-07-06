pub use builder::ContextBuilder;
use lune_std::context::GlobalsContext;
use std::path::PathBuf;

mod builder;

pub(crate) struct Context {
    pub(crate) lune_ctx: GlobalsContext,
    pub(crate) luau_input: Option<PathBuf>,
    pub(crate) javascript_inputs: Vec<PathBuf>,
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
            luau_input: value.luau_input,
            javascript_inputs: value.javascript_inputs,
        }
    }
}

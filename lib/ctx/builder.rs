use lune_std::context::GlobalsContextBuilder;
use std::path::PathBuf;

#[derive(Default)]
pub struct ContextBuilder {
    pub(crate) lune_ctx_builder: Option<GlobalsContextBuilder>,
    pub(crate) luau_input: Option<PathBuf>,
    pub(crate) javascript_inputs: Vec<PathBuf>,
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
        let input = input.into();
        let Some(ext) = input.extension() else {
            panic!("The provided input file for luau must have a .luau extension");
        };

        if ext != "luau" {
            panic!("The provided input file for luau has '{ext:?}' as its extension");
        }

        self.luau_input = Some(input);
        self
    }

    pub fn with_javascript_inputs(mut self, inputs: Vec<PathBuf>) -> Self {
        for path in &inputs {
            let Some(ext) = path.extension() else {
                panic!("The provided input file for javascript must have a .js extension");
            };

            if ext != "js" {
                panic!("The provided input file for javascript has '{ext:?}' as its extension");
            }
        }

        self.javascript_inputs.append(&mut inputs.clone());
        self
    }
}

use luneweb::{app::App, ctx::ContextBuilder, util::lune_ctx};
use std::path::PathBuf;

mod cli;

async fn start_application(input: PathBuf, javascript_inputs: Vec<PathBuf>) {
    App::from(
        ContextBuilder::new()
            .lune_ctx(lune_ctx().expect("Failed to create GlobalsContextBuilder"))
            .js_inputs(javascript_inputs)
            .luau_input(&input),
    )
    .run()
    .await
    .expect("Failed to run application");
}

#[tokio::main]
async fn main() {
    cli::init().await
}

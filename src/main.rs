use luneweb::{app::App, ctx::ContextBuilder, util::lune_ctx};
use std::path::PathBuf;

mod cli;

async fn start_application(input: PathBuf, javascript_inputs: Vec<PathBuf>) {
    App::from(
        ContextBuilder::new()
            .with_lune_ctx(lune_ctx().expect("Failed to create GlobalsContextBuilder"))
            .with_luau_input(&input)
            .with_javascript_inputs(javascript_inputs),
    )
    .run()
    .await
    .expect("Failed to run application");
}

#[tokio::main]
async fn main() {
    cli::init().await
}

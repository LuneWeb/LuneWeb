use app::App;
use ctx::{Context, ContextBuilder};
use lune_std::context::GlobalsContextBuilder;
use util::Error as LuneWebError;

mod app;
mod ctx;
mod util;

fn lune_ctx() -> Result<GlobalsContextBuilder, LuneWebError> {
    let mut builder = GlobalsContextBuilder::new();

    // Inject lune standard libraries
    lune_std::inject_libraries(&mut builder)?;

    Ok(builder)
}

#[tokio::main]
async fn main() {
    let lune_ctx = lune_ctx().expect("Failed to create GlobalsContextBuilder");

    App::from(ContextBuilder::new().with_lune_ctx(lune_ctx))
        .run()
        .await
        .expect("Failed to run application");
}

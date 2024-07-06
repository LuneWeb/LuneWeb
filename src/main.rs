use app::App;
use ctx::{Context, ContextBuilder};
use util::{lune_ctx, Error as LuneWebError};

mod app;
mod ctx;
mod util;

#[tokio::main]
async fn main() {
    App::from(
        ContextBuilder::new()
            .with_lune_ctx(lune_ctx().expect("Failed to create GlobalsContextBuilder")),
    )
    .run()
    .await
    .expect("Failed to run application");
}

use include_dir::include_dir;
use luneweb::{include_luau, App, Context, LuneWebError};

#[tokio::main]
async fn main() -> Result<(), LuneWebError> {
    let luau_dir = include_dir!("examples/luau");

    App::new(Context::new().javascript_dir(include_dir!("examples/javascript")))
        .run()
        .await
}

use include_dir::include_dir;
use luneweb::{include_luau, App, Context, LuneWebError};

#[tokio::main]
async fn main() -> Result<(), LuneWebError> {
    App::new(
        Context::new()
            .luau_ctx(include_luau!(
                include_dir!("examples/luau"),
                "examples/luau"
            ))
            .javascript_dir(include_dir!("examples/javascript")),
    )
    .run()
    .await
}

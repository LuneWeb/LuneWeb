use include_dir::include_dir;
use luneweb::{App, Context, LuneWebError};

fn main() -> Result<(), LuneWebError> {
    App::new(
        Context::new()
            .luau_dir(include_dir!("examples/luau"))?
            .javascript_dir(include_dir!("examples/javascript"))?,
    )
    .run()
}

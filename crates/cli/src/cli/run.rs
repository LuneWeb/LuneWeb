use std::{path::PathBuf, rc::Rc};

use lune_std::context::GlobalsContextBuilder;
use luneweb_app::App;
use mlua_luau_scheduler::Scheduler;
use tokio::fs;

use crate::config::LunewebConfig;

use super::set_cwd;

pub async fn run(dir: Option<PathBuf>) {
    let cwd = set_cwd(dir);
    let config = LunewebConfig::from(cwd.clone());

    let lua = mlua::Lua::new();
    let app = App::new(config.clone().into()).expect("Failed to create app");

    app.into_global(&lua)
        .expect("Failed to inject app into lua globals");

    let mut ctx = GlobalsContextBuilder::new();
    lune_std::inject_libraries(&mut ctx).unwrap();

    let ctx = ctx.build();

    lune_std::inject_globals(&lua, &ctx).unwrap();
    lua.sandbox(true).unwrap();

    // sandboxing makes already inserted globals read-only
    // so we create the _G global again
    lune_std::LuneStandardGlobal::GTable
        .create(&lua, &ctx)
        .unwrap();

    let lua = {
        let rc = Rc::new(lua);
        rc.set_app_data(Rc::downgrade(&rc));
        rc
    };
    let scheduler = Scheduler::new(&lua);

    let luau_code = {
        let bytes_content = fs::read(&config.app.luau)
            .await
            .expect("Failed to read luau file");
        let content = String::from_utf8(bytes_content).expect("String::from_utf8 failed");

        lua.load(content)
            .set_name(config.app.luau.to_string_lossy())
    };

    scheduler
        .push_thread_back(luau_code, ())
        .expect("Failed to push luau thread back");

    scheduler.run().await;
}

use std::{path::PathBuf, rc::Rc};

use lune_std::context::GlobalsContextBuilder;
use luneweb_app::{config::AppConfig, App};
use mlua_luau_scheduler::Scheduler;
use tokio::fs;

use crate::config::LunewebConfig;

use super::set_cwd;

pub async fn run(dir: Option<PathBuf>) {
    let cwd = set_cwd(dir);
    let config = LunewebConfig::from(cwd.clone());

    let config_dev = config.dev.unwrap_or(crate::config::LunewebConfigDev {
        url: Some("http://localhost:5173/".into()),
    });

    let title = match cwd.file_stem() {
        Some(stem) => stem.to_string_lossy(),
        None => "LuneWeb".into(),
    };

    let app_dev = config
        .app
        .unwrap_or(crate::config::LunewebConfigApp { luau: None });

    let lua = mlua::Lua::new();
    let app = App::new(AppConfig {
        window_title: title.to_string(),
        url: config_dev
            .url
            .clone()
            .expect("Expected url from luneweb.toml"),
    })
    .expect("Failed to create app");

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

    if let Some(luau_path) = &app_dev.luau {
        let luau_code = {
            let bytes_content = fs::read(luau_path).await.unwrap();
            let content = String::from_utf8(bytes_content).unwrap();

            lua.load(content).set_name(luau_path.to_string_lossy())
        };

        scheduler.push_thread_back(luau_code, ()).unwrap();
    }

    scheduler.run().await;
}

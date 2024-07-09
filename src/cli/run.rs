use std::{path::PathBuf, rc::Rc};

use lune_std::context::GlobalsContextBuilder;
use mlua_luau_scheduler::Scheduler;
use tokio::{fs, process::Command};

use crate::{app::App, config::LunewebConfig, message, ONLOAD_TX};

use super::set_cwd;

pub async fn run(dir: Option<PathBuf>) {
    let cwd = set_cwd(dir);
    let config = LunewebConfig::from(cwd.clone());

    let config_dev = config.dev.unwrap_or(crate::config::LunewebConfigDev {
        url: Some("http://localhost:5173/".into()),
        pkg_manager: None,
        pkg_install: None,
    });

    let title = match cwd.file_stem() {
        Some(stem) => stem.to_string_lossy(),
        None => "LuneWeb".into(),
    };

    let app_dev = config
        .app
        .unwrap_or(crate::config::LunewebConfigApp { luau: None });

    if let Some(pkg_manager) = config_dev.pkg_manager {
        let mut command = Command::new(pkg_manager);

        if let Some(arg) = config_dev.pkg_install {
            command.arg(arg);
        }

        command
            .spawn()
            .expect("Failed to install node_modules")
            .wait_with_output()
            .await
            .unwrap();
    }

    let mut ctx = GlobalsContextBuilder::new();
    lune_std::inject_libraries(&mut ctx).unwrap();
    crate::inject_libraries(&mut ctx).unwrap();

    let lua = mlua::Lua::new();
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

    let app = App::new(
        |builder_window| builder_window.with_title(title.clone()),
        |builder_webview| {
            builder_webview
                .with_initialization_script(&format!("{{ {0} }}", message::JS_IMPL))
                .with_url(
                    config_dev
                        .url
                        .clone()
                        .expect("Expected url from luneweb.toml"),
                )
                .with_ipc_handler(|_| {
                    ONLOAD_TX.with_borrow(|tx| {
                        if tx.receiver_count() > 0 {
                            tx.send(()).unwrap();
                        }
                    });
                })
        },
    );

    if let Some(luau_path) = &app_dev.luau {
        let luau_code = {
            let bytes_content = fs::read(luau_path).await.unwrap();
            let content = String::from_utf8(bytes_content).unwrap();

            lua.load(content).set_name(luau_path.to_string_lossy())
        };

        scheduler.push_thread_back(luau_code, ()).unwrap();
    }

    let logic_thread = lua.create_thread(app.init_logic(&lua)).unwrap();
    scheduler.push_thread_front(logic_thread, ()).unwrap();
    scheduler.run().await;
}

use std::{path::PathBuf, process, rc::Rc};

use lune_std::context::GlobalsContextBuilder;
use mlua_luau_scheduler::Scheduler;
use tokio::{fs, process::Command};

use crate::{config::LunewebConfig, logic, message, APP, EVENT_LOOP, ONLOAD_TX};

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

    let builder_window = window_builder!().with_title(title);
    let window = Rc::new(
        EVENT_LOOP
            .with_borrow(|event_loop| builder_window.build(event_loop))
            .unwrap(),
    );

    let builder_webview = webview_builder!(window)
        .with_initialization_script(&format!("{{ {0} }}", message::JS_IMPL))
        .with_url(config_dev.url.expect("Expected url from luneweb.toml"))
        .with_ipc_handler(|_| {
            ONLOAD_TX.with_borrow(|tx| {
                if tx.receiver_count() > 0 {
                    tx.send(()).unwrap();
                }
            });
        });
    let webview = Rc::new(builder_webview.build().unwrap());

    if let Some(luau_path) = &app_dev.luau {
        let luau_code = {
            let bytes_content = fs::read(luau_path).await.unwrap();
            let content = String::from_utf8(bytes_content).unwrap();

            lua.load(content).set_name(luau_path.to_string_lossy())
        };

        scheduler.push_thread_back(luau_code, ()).unwrap();
    }

    // main logic
    let logic_function = lua
        .create_async_function(move |_, _: ()| {
            let window = Rc::clone(&window);

            async move {
                logic(window).await?;

                process::exit(0);

                #[allow(unreachable_code)]
                Ok(())
            }
        })
        .unwrap();

    APP.set(crate::App {
        webview: Some(webview),
    });

    let logic_thread = lua.create_thread(logic_function).unwrap();
    scheduler.push_thread_front(logic_thread, ()).unwrap();
    scheduler.run().await;
}

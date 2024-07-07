use clap::Parser;
use lune_std::context::GlobalsContextBuilder;
use mlua_luau_scheduler::Scheduler;
use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
    rc::Rc,
};
use tokio::{fs, process::Command};

use crate::{config::LunewebConfig, logic, webview_builder, window_builder, EVENT_LOOP};
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Run { dir: Option<PathBuf> },
    Build,
}

fn set_cwd(dir: Option<PathBuf>) -> PathBuf {
    let cwd = current_dir().unwrap();
    let cwd = match dir {
        Some(dir) => cwd.join(dir),
        None => cwd,
    };

    set_current_dir(&cwd).unwrap();
    cwd
}

pub async fn init() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Run { dir } => {
            let cwd = set_cwd(dir);
            let config = LunewebConfig::from(cwd.clone());

            let _vite_process = Command::new("npx").arg("vite").spawn().expect("Failed to run command 'npx vite' make sure to have node js installed and have installed vite in your dev dependencies");

            let mut ctx = GlobalsContextBuilder::new();
            lune_std::inject_libraries(&mut ctx).unwrap();

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

            let builder_window = window_builder!();
            let window = Rc::new(
                EVENT_LOOP
                    .with_borrow(|event_loop| builder_window.build(event_loop))
                    .unwrap(),
            );

            let builder_webview = webview_builder!(window).with_url(config.dev.url);
            let _webview = builder_webview.build().unwrap();

            let luau_code = {
                let bytes_content = fs::read(&config.app.luau).await.unwrap();
                let content = String::from_utf8(bytes_content).unwrap();

                lua.load(content)
                    .set_name(config.app.luau.to_string_lossy())
            };
            scheduler.push_thread_front(luau_code, ()).unwrap();

            // main logic
            let logic_function = lua
                .create_async_function(move |_, _: ()| {
                    let window = Rc::clone(&window);

                    async move { logic(window).await }
                })
                .unwrap();

            let logic_thread = lua.create_thread(logic_function).unwrap();
            scheduler.push_thread_front(logic_thread, ()).unwrap();
            scheduler.run().await;
        }
        SubCommand::Build => {
            unimplemented!()
        }
    }
}

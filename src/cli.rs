use clap::Parser;
use lune_std::context::GlobalsContextBuilder;
use mlua_luau_scheduler::Scheduler;
use std::{env::current_dir, path::PathBuf, rc::Rc};
use tokio::fs;

use crate::{config::LunewebConfig, logic, webview_builder, window_builder, EVENT_LOOP};
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Run,
    Build,
}

pub async fn init() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Run => {
            let config = LunewebConfig::from(current_dir().unwrap());

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
            let webview = builder_webview.build().unwrap();

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

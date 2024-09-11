use std::{path::PathBuf, rc::Rc};

use luneweb_rs::classes::eventloop::EventLoop;
use mlua_luau_scheduler::Scheduler;
use tokio::fs::{self, canonicalize};

use crate::VERSION;

pub async fn run(src: PathBuf) -> Result<(), mlua::Error> {
    let lua = Rc::new(mlua::Lua::new());

    lua.set_app_data(Rc::downgrade(&lua));
    lua.set_app_data(Vec::<String>::new());

    let scheduler = Scheduler::new(&lua);
    let compiler = mlua::Compiler::new()
        .set_coverage_level(2)
        .set_debug_level(2)
        .set_optimization_level(1);

    let chunk = lua
        .load(fs::read(&src).await?)
        .set_compiler(compiler)
        .set_name(canonicalize(src).await?.to_string_lossy());

    luneweb_std::inject_globals(&lua)?;
    lune_std::inject_globals(&lua)?;

    let _version = lune_std::LuneStandardGlobal::Version;
    lua.globals()
        .set(_version.name(), format!("luneweb {VERSION}"))?;

    lua.sandbox(true)?;

    // sandboxing sets existing globals to read-only
    // so we have to inject _G after sandboxing
    let _g = lune_std::LuneStandardGlobal::GTable;
    lua.globals().set(_g.name(), _g.create(&lua)?)?;

    EventLoop::new().finalize(&lua, &scheduler);

    scheduler.push_thread_back(chunk, ())?;
    scheduler.run().await;

    Ok(())
}

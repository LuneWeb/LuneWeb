use std::path::PathBuf;

use luneweb_rs::classes::eventloop::EventLoop;
use mlua_luau_scheduler::Scheduler;
use tokio::fs::{self, canonicalize};

use crate::VERSION;

pub async fn run(src: PathBuf) -> Result<(), mlua::Error> {
    let lua = mlua::Lua::new();
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

    // lune only allows dots and digits in version string
    let version = VERSION
        .chars()
        .take_while(is_valid_version_char)
        .collect::<String>();

    lune_std::set_global_version(&lua, version);

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

fn is_valid_version_char(c: &char) -> bool {
    matches!(c, '0'..='9' | '.')
}

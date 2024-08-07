use luneweb_rs::classes::eventloop::EventLoop;
use mlua_luau_scheduler::Scheduler;
use tokio::fs::{self, canonicalize};

const SCRIPT_PATH: &str = "examples/init.luau";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main(flavor = "multi_thread")]
async fn main() -> mlua::Result<()> {
    let lua = mlua::Lua::new();
    let scheduler = Scheduler::new(&lua);
    let compiler = mlua::Compiler::new()
        .set_coverage_level(2)
        .set_debug_level(2)
        .set_optimization_level(1);

    let chunk = lua
        .load(fs::read(SCRIPT_PATH).await?)
        .set_compiler(compiler)
        .set_name(canonicalize(SCRIPT_PATH).await?.to_string_lossy());

    luneweb_std::inject_globals(&lua)?;
    lune_std::inject_globals(&lua)?;
    lune_std::set_global_version(&lua, VERSION);

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

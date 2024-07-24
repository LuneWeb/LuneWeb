use luneweb_rs::classes::eventloop::EventLoop;
use luneweb_std::StandardLibrary;
use mlua_luau_scheduler::Scheduler;

const SCRIPT: &str = include_str!("init.luau");

#[tokio::main(flavor = "multi_thread")]
async fn main() -> mlua::Result<()> {
    let lua = mlua::Lua::new();
    let globals = lua.globals();
    let scheduler = Scheduler::new(&lua);
    let compiler = mlua::Compiler::new()
        .set_coverage_level(2)
        .set_debug_level(2)
        .set_optimization_level(1);

    let chunk = lua.load(SCRIPT).set_compiler(compiler);

    luneweb_std::inject_globals(&lua, &globals)?;

    EventLoop::new().finalize(&lua, &scheduler);

    scheduler.push_thread_back(chunk, ())?;
    scheduler.run().await;

    Ok(())
}

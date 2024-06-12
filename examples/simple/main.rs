use std::{env::current_dir, path::PathBuf, rc::Rc};

use lune_std::context::GlobalsContextBuilder;
use mlua::Lua;
use mlua_luau_scheduler::Scheduler;

fn create_lua() -> Result<Rc<Lua>, mlua::Error> {
    let lua = Rc::new(Lua::new());
    let mut ctx = GlobalsContextBuilder::new();

    luneweb::patch_lua(&lua);
    luneweb::inject_libraries(&mut ctx)?;
    lune_std::inject_libraries(&mut ctx)?;

    lua.sandbox(true)?;

    lune_std::inject_globals(&lua, &ctx.build())?;

    Ok(lua)
}

#[tokio::main]
async fn main() -> Result<(), mlua::Error> {
    let dir = current_dir()?.join("examples/simple/main.luau");
    let src = include_str!("main.luau");

    let lua = create_lua()?;
    let sched = Scheduler::new(&lua);
    let chunk = lua.load(src).set_name(dir.to_string_lossy());

    sched.push_thread_back(chunk, ())?;
    sched.run().await;

    Ok(())
}

use crate::libraries::LuneWebLibraries;
use lune_std::context::GlobalsContextBuilder;
use mlua::prelude::*;
use std::rc::Rc;

/**
    Create a Weak\<Lua> reference for the Lua struct

    This is required for most luneweb and lune libraries
*/
pub fn patch_lua(lua: &Rc<Lua>) {
    lua.set_app_data(Rc::downgrade(&lua));
}

/**
    Creates a GlobalsContextBuilder struct and injects LuneWeb libraries into it.
*/
pub fn create_and_inject_globals() -> Result<GlobalsContextBuilder, LuaError> {
    let mut builder = GlobalsContextBuilder::new();

    builder.with_alias("luneweb", |modules| {
        for lib in LuneWebLibraries::ALL {
            modules.insert(lib.name(), lib.module_creator());
        }

        Ok(())
    })?;

    Ok(builder)
}

/**
    Injects LuneWeb libraries into the provided GlobalsContextBuilder struct.
*/
pub fn inject_globals(globals_ctx_builder: &mut GlobalsContextBuilder) -> Result<(), LuaError> {
    globals_ctx_builder.with_alias("luneweb", |modules| {
        for lib in LuneWebLibraries::ALL {
            modules.insert(lib.name(), lib.module_creator());
        }

        Ok(())
    })
}

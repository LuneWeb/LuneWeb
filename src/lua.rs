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
    Injects LuneWeb libraries into the provided GlobalsContextBuilder struct.
*/
pub fn inject_libraries(globals_ctx_builder: &mut GlobalsContextBuilder) -> Result<(), LuaError> {
    globals_ctx_builder.with_alias("luneweb", |modules| {
        for lib in LuneWebLibraries::ALL {
            modules.insert(lib.name(), lib.module_creator());
        }

        Ok(())
    })
}

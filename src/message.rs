pub fn create_luneweb(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    lune_utils::TableBuilder::new(lua)?.build_readonly()
}

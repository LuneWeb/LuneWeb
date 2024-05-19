use mlua::prelude::*;
use tokio::sync::watch;

pub struct LuaConnection {
    pub shutdown_tx: watch::Sender<bool>,
}

impl LuaConnection {
    pub fn new() -> Self {
        Self {
            shutdown_tx: watch::Sender::new(false),
        }
    }
}

impl LuaUserData for LuaConnection {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("connected", |_, this| {
            let is_disconnected = *this.shutdown_tx.subscribe().borrow();
            Ok(!is_disconnected)
        });

        fields.add_field_method_get("listeners", |_, this| Ok(this.shutdown_tx.receiver_count()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("disconnect", |_, this, _: ()| {
            this.shutdown_tx.send(true).into_lua_err()
        });
    }
}

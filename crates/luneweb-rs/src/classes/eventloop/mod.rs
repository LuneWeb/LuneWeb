use mlua::IntoLua;
use mlua_luau_scheduler::Scheduler;
use tao::event_loop::{EventLoop as _EventLoop, EventLoopBuilder as _EventLoopBuilder};

use super::window::Window;

mod logic;

pub struct EventLoop {
    pub(super) inner: _EventLoop<()>,
    pub(super) windows: Vec<Window>,
}

impl EventLoop {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: _EventLoopBuilder::new().build(),
            windows: Vec::new(),
        }
    }

    pub fn finalize(self, lua: &mlua::Lua, scheduler: &Scheduler) {
        scheduler
            .push_thread_front(self.lua_function(lua).as_function().unwrap(), ())
            .expect("Failed to push EventLoop thread into Scheduler");

        lua.set_app_data(self);
    }

    fn lua_function<'lua>(&'lua self, lua: &'lua mlua::Lua) -> mlua::Value {
        let wrapped = mlua::Function::wrap_async(logic::lua_run);

        wrapped
            .into_lua(lua)
            .expect("Failed to wrapped Rust function into Lua value")
    }
}

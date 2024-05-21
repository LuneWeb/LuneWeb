use lune_utils::TableBuilder;
use mlua::prelude::*;
use mlua_luau_scheduler::{LuaSchedulerExt, LuaSpawnExt};
use once_cell::sync::Lazy;
use std::{borrow::Borrow, cell::RefCell, rc::Weak, sync::Mutex, time::Duration};
use tao::{
    event::WindowEvent,
    event_loop::{EventLoop, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

use crate::
    classes::{connection::LuaConnection, windowid::LuaWindowId};

#[derive(Clone, Copy)]
pub enum LuaEvent {
    CloseRequested,
}

impl<'lua> IntoLua<'lua> for LuaEvent {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        match self {
            LuaEvent::CloseRequested => "CloseRequested".into_lua(lua),
        }
    }
}

thread_local! {
    pub static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoopBuilder::new().build());
}

// Some say we shouldn't do stuff like this
// but I like it! its simple and it just works.
pub static EVENT_LOOP_ACTIVE: Mutex<bool> = Mutex::new(false);

pub struct EventLoopInfo {
    pub window_id: Option<WindowId>,
    pub lua_event: Option<LuaEvent>,
}

pub static EVENT_LOOP_SENDER: Lazy<tokio::sync::watch::Sender<EventLoopInfo>> = Lazy::new(|| {
    let init = EventLoopInfo {
        window_id: None,
        lua_event: None,
    };

    tokio::sync::watch::Sender::new(init)
});

pub fn create(lua: &Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function(
            "subscribe",
            |lua, (lua_window_id, callback): (LuaValue, LuaFunction)| {
                let lua_window_id = lua_window_id
                    .as_userdata()
                    .expect("Couldn't get userdata from first parameter")
                    .borrow::<LuaWindowId>()
                    .expect("Couldn't get window id from first parameter");

                let inner_lua = lua
                    .app_data_ref::<Weak<Lua>>()
                    .expect("Missing weak lua ref")
                    .upgrade()
                    .expect("Lua was dropped unexpectedly");

                let window_id = lua_window_id.0;
                let key_callback = lua.create_registry_value(callback)?;

                let connection = LuaConnection::new();
                let mut shutdown_rx = connection.shutdown_tx.subscribe();

                let event_loop_active = EVENT_LOOP_ACTIVE
                    .lock()
                    .expect("Failed to lock EVENT_LOOP_ACTIVE mutex");

                lua.spawn_local(async move {
                    let mut event_listener = EVENT_LOOP_SENDER.subscribe();

                    loop {                        
                        tokio::select! {
                            Ok(_) = shutdown_rx.changed() => break,
                            _ = event_listener.changed() => {},
                        }

                        let info = event_listener.borrow_and_update();

                        if let Some(event_window_id) = info.window_id {
                            if event_window_id == window_id {
                                let callback = inner_lua
                                    .registry_value::<LuaFunction>(&key_callback)
                                    .unwrap();

                                let thread = inner_lua.create_thread(callback).unwrap();
                                inner_lua
                                    .as_ref()
                                    .push_thread_back(thread, info.lua_event)
                                    .unwrap();
                            }
                        }
                    }
                });

                if !*event_loop_active {
                    // event_loop.new() will use the mutex so we have to drop it
                    drop(event_loop_active);
                    
                    // activate the main event loop automatically :)
                    lua.load(r#"
                    require("@luneweb/event_loop").new()
                    "#).eval::<()>().expect("Failed to create event loop automatically");
                }

                Ok(connection)
            },
        )?
        .with_function("new", |lua: &Lua, _: ()| {
            let mut event_loop_active = EVENT_LOOP_ACTIVE
                .lock()
                .expect("Failed to lock EVENT_LOOP_ACTIVE mutex");
        
            if *event_loop_active {
                return Err(LuaError::RuntimeError("Subscribing to the event loop for the first time will automatically create an event loop for you, if you're creating the event loop manually you should stop".into()));
            } else {
                *event_loop_active = true;
            }
        
            lua.spawn_local(async move {
                loop {
                    let info: EventLoopInfo = EVENT_LOOP.with(|event_loop| {
                        let mut event_loop = event_loop.borrow_mut();
        
                        let mut _window_id: Option<WindowId> = None;
                        let mut lua_event: Option<LuaEvent> = None;
        
                        event_loop.run_return(|event, _, flow| {
                            match event.borrow() {
                                tao::event::Event::WindowEvent {
                                    window_id, event, ..
                                } => {
                                    // HANDLE WINDOW EVENTS
                                    _window_id = Some(*window_id);
                                    lua_event = match event {
                                        WindowEvent::CloseRequested => Some(LuaEvent::CloseRequested),
                                        _ => None,
                                    }
                                }
                                _ => {}
                            }
        
                            let can_exit = match event.borrow() {
                                tao::event::Event::MainEventsCleared => true,
                                tao::event::Event::LoopDestroyed => true,
                                tao::event::Event::WindowEvent { .. } => true,
                                tao::event::Event::UserEvent(_) => true,
                                _ => false,
                            } || EVENT_LOOP_SENDER.receiver_count() == 0;
        
                            if can_exit {
                                *flow = tao::event_loop::ControlFlow::Exit;
                            }
                        });
        
                        EventLoopInfo {
                            window_id: _window_id,
                            lua_event,
                        }
                    });
        
                    if !EVENT_LOOP_SENDER.is_closed() {
                        EVENT_LOOP_SENDER.send(info).into_lua_err().unwrap();
                    } else {
                        break;
                    }
        
                    tokio::time::sleep(Duration::from_millis(16)).await;
                }
        
                *EVENT_LOOP_ACTIVE
                    .lock()
                    .expect("Failed to lock EVENT_LOOP_ACTIVE mutex") = false;
            });
        
            Ok(())
        })?
        .build_readonly()
}

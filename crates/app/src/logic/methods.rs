use std::{rc::Rc, time::Duration};

use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
};

use crate::App;

pub fn implement_lua_methods<'lua, M: mlua::UserDataMethods<'lua, App>>(methods: &mut M) {
    #[allow(unreachable_code)]
    methods.add_async_method("run", |_, app, _: ()| async {
        let event_loop_cell = &app.event_loop;
        let window = Rc::clone(&app.window);

        loop {
            let mut exit = false;

            event_loop_cell
                .borrow_mut()
                .run_return(|event, _target, control_flow| {
                    match event {
                        Event::WindowEvent {
                            window_id: _,
                            event: WindowEvent::CloseRequested,
                            ..
                        } => {
                            window.set_visible(false);
                            exit = true;
                        }
                        Event::RedrawRequested(_) => {
                            window.request_redraw();
                        }
                        _ => {}
                    };

                    let can_exit = matches!(
                        event,
                        tao::event::Event::MainEventsCleared
                            | tao::event::Event::LoopDestroyed
                            | tao::event::Event::WindowEvent { .. }
                            | tao::event::Event::UserEvent(_)
                    );

                    if can_exit {
                        *control_flow = ControlFlow::Exit;
                    }
                });

            if exit {
                break;
            }

            tokio::time::sleep(Duration::from_millis(16)).await;
        }

        Ok(std::process::exit(0))
    });
}

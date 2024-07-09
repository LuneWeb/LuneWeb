use std::{cell::RefCell, process, rc::Rc};

use tao::window::{Window, WindowBuilder};
use wry::{WebView, WebViewBuilder};

use crate::{logic, EVENT_LOOP};

thread_local! {
    pub static APP: RefCell<Option<App>> = const { RefCell::new(None) };
}

pub struct App {
    pub window: Rc<Window>,
    pub webview: Rc<WebView>,
}

impl App {
    pub fn new(
        window_callback: impl Fn(WindowBuilder) -> WindowBuilder,
        webview_callback: impl Fn(WebViewBuilder) -> WebViewBuilder,
    ) -> Self {
        let builder_window = window_callback(window_builder!());
        let window = Rc::new(
            EVENT_LOOP
                .with_borrow(|event_loop| builder_window.build(event_loop))
                .unwrap(),
        );

        let builder_webview = webview_callback(webview_builder!(window));
        let webview = Rc::new(builder_webview.build().unwrap());

        Self { window, webview }
    }

    pub fn init_logic(self, lua: &mlua::Lua) -> mlua::Function {
        // main logic
        let inner_window = Rc::clone(&self.window);
        let logic_function = lua
            .create_async_function(move |_, _: ()| {
                let window = Rc::clone(&inner_window);

                async move {
                    logic(window).await?;

                    process::exit(0);

                    #[allow(unreachable_code)]
                    Ok(())
                }
            })
            .unwrap();

        APP.set(Some(self));

        logic_function
    }
}

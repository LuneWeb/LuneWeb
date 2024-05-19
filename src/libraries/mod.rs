use lune_std::context::LuneModuleCreator;

mod event_loop;
mod webview;
mod window;

pub enum LuneWebLibraries {
    Window,
    WebView,
    EventLoop,
}

impl LuneWebLibraries {
    pub const ALL: &'static [Self] = &[Self::Window, Self::WebView, Self::EventLoop];

    pub fn name(&self) -> &str {
        match self {
            Self::Window => "window",
            Self::WebView => "webview",
            Self::EventLoop => "event_loop",
        }
    }

    pub fn module_creator(&self) -> LuneModuleCreator {
        match self {
            Self::Window => LuneModuleCreator::LuaTable(window::create),
            Self::WebView => LuneModuleCreator::LuaTable(webview::create),
            Self::EventLoop => LuneModuleCreator::LuaTable(event_loop::create),
        }
    }
}

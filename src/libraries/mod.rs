use lune_std::context::LuneModuleCreator;

#[cfg(feature = "webview")]
pub mod webview;

pub mod event_loop;
pub mod window;

#[rustfmt::skip]
pub enum LuneWebLibraries {
    #[cfg(feature = "webview")] WebView,
    Window,
    EventLoop,
}

impl LuneWebLibraries {
    #[rustfmt::skip]
    pub const ALL: &'static [Self] = &[
        #[cfg(feature = "webview")] Self::WebView,
        Self::Window,
        Self::EventLoop,
    ];

    #[rustfmt::skip]
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "webview")] Self::WebView => "webview",
            Self::Window => "window",
            Self::EventLoop => "event_loop",
        }
    }

    #[rustfmt::skip]
    pub fn module_creator(&self) -> LuneModuleCreator {
        match self {
            #[cfg(feature = "webview")] Self::WebView => LuneModuleCreator::LuaTable(webview::create),
            Self::Window => LuneModuleCreator::LuaTable(window::create),
            Self::EventLoop => LuneModuleCreator::LuaTable(event_loop::create),
        }
    }
}

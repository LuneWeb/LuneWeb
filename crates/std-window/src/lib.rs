use std::rc::Rc;

use luneweb_rs::classes::window::Window;
use message::LuaMessage;
use mlua::{ExternalResult, IntoLua};
use tao::window::WindowId;
use tokio::sync::watch;
use webview::LuaWebview;

pub mod message;
mod serde;
pub mod webview;

/**

### Macro

`let <identifier> << <&Lua>, <WindowId>`

### Example

```rust
//                        &Lua, WindowId
inner_window(let window << lua, id)
```

 */
#[macro_export]
macro_rules! inner_window {
    (let $var:ident << $lua:expr, $id:expr) => {
        use luneweb_rs::classes::eventloop::EventLoop;

        let event_loop = $lua
            .app_data_ref::<EventLoop>()
            .expect("Coulnd't get reference to EventLoop");

        let $var = event_loop
            .get_window($id)
            .expect("Couldn't find Window in EventLoop");
    };
}

pub struct LuaWindowConfig {
    pub title: String,
    pub url: String,
    pub dev: bool,
}

impl<'lua> mlua::FromLua<'lua> for LuaWindowConfig {
    fn from_lua(value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> mlua::Result<Self> {
        let t = value
            .as_table()
            .ok_or(mlua::Error::runtime("WindowConfig should be a table"))?;
        let title = t.get("title").unwrap_or("LuneWeb".to_string());
        let url = t.get("url").unwrap_or("about:blank".to_string());
        let dev = t.get("dev").unwrap_or(false);

        Ok(Self { title, url, dev })
    }
}

pub struct LuaWindow {
    pub(crate) id: WindowId,
    pub message: Rc<LuaMessage>,
    pub webview: Rc<LuaWebview>,
}

impl LuaWindow {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(lua: &mlua::Lua, config: LuaWindowConfig) -> mlua::Result<mlua::Value> {
        let window = Window::new(lua)
            .into_lua_err()?
            .with_title(&config.title)
            .with_webview(config.dev, move |x| x.with_url(&config.url))
            .into_lua_err()?;
        let id = window.inner.id();

        if let Some(webview) = &window.webview {
            webview.toggle_dev(config.dev);
        }

        window.finalize(lua).into_lua_err()?;

        Self {
            id,
            message: Rc::new(LuaMessage {
                id,
                tx: Rc::new(watch::Sender::new(String::new())),
            }),
            webview: Rc::new(LuaWebview { id }),
        }
        .into_lua(lua)
    }
}

impl mlua::UserData for LuaWindow {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("message", |_, this| Ok(Rc::clone(&this.message)));
        fields.add_field_method_get("webview", |_, this| Ok(Rc::clone(&this.webview)));

        fields.add_field_method_get("title", |lua, this| {
            inner_window!(let window << lua, this.id);

            Ok(window.inner.title())
        });
        fields.add_field_method_set("title", |lua, this, title: String| {
            inner_window!(let window << lua, this.id);

            window.inner.set_title(&title);

            Ok(())
        });

        fields.add_field_method_get("visible", |lua, this| {
            inner_window!(let window << lua, this.id);

            Ok(window.inner.is_visible())
        });
        fields.add_field_method_set("visible", |lua, this, visible: bool| {
            inner_window!(let window << lua, this.id);

            window.inner.set_visible(visible);

            Ok(())
        });
    }
}

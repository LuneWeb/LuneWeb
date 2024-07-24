use luneweb_rs::classes::{eventloop::EventLoop, window::Window};
use mlua::{ExternalResult, IntoLua};
use tao::window::WindowId;

/**

### Macro

`let <identifier> << <&Lua>, <WindowId>`

### Example

```rust
//                        &Lua, WindowId
inner_window(let window << lua, this.id)
```

 */
macro_rules! inner_window {
    (let $var:ident << $lua:expr, $id:expr) => {
        let event_loop = $lua
            .app_data_ref::<EventLoop>()
            .expect("Coulnd't get reference to EventLoop");

        let $var = event_loop
            .get_window($id)
            .expect("Couldn't find Window in EventLoop");
    };
}

pub struct LuaWindow {
    id: WindowId,
}

impl LuaWindow {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(lua: &mlua::Lua, title: String) -> mlua::Result<mlua::Value> {
        let window = Window::new(lua)
            .into_lua_err()?
            .with_title(&title)
            .with_webview(|x| x.with_url("https://luneweb.github.io/docs/"))
            .into_lua_err()?;
        let id = window.inner.id();

        window.finalize(lua).into_lua_err()?;

        Self { id }.into_lua(lua)
    }
}

impl mlua::UserData for LuaWindow {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
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
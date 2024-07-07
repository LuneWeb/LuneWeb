use crate::APP;

pub const JS_IMPL: &str = include_str!(".js");

pub fn create(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    lune_utils::TableBuilder::new(lua)?
        .with_function("send", |lua, message: mlua::Value| {
            let json = lune_std_serde::encode(
                message,
                lua,
                lune_std_serde::EncodeDecodeConfig {
                    format: lune_std_serde::EncodeDecodeFormat::Json,
                    pretty: false,
                },
            )?;

            APP.with_borrow(|app| {
                if let Some(webview) = &app.webview {
                    let string_json = json.to_string_lossy();

                    webview
                        .evaluate_script(&format!("window._sendMessage({string_json})"))
                        .unwrap();

                    Ok(())
                } else {
                    Err(mlua::Error::RuntimeError("WebView not available".into()))
                }
            })
        })?
        .build_readonly()
}

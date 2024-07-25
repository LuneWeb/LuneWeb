use super::WebView;

impl WebView {
    pub fn call_js_channel(&self, channel: String, value: String) -> Result<(), String> {
        match self.inner.evaluate_script(&format!(
            "window.luneweb.callChannel('{channel}', '{value}')"
        )) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

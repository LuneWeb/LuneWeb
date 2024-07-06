use include_dir::Dir;

#[derive(Default)]
pub struct Context {
    pub(crate) javascript: Option<Dir<'static>>,
}

impl Context {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn javascript_dir(mut self, dir: Dir<'static>) -> Self {
        self.javascript = Some(dir);
        self
    }
}

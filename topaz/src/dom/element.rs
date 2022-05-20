#[derive(Debug)]
pub struct Element {
    pub inner: web_sys::Element,
}

impl Element {
    pub fn new(inner: web_sys::Element) -> Self {
        Self { inner }
    }
}
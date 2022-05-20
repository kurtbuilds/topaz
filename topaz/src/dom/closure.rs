use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

pub struct EventListener {
    pub(crate) inner: Closure<dyn FnMut(web_sys::Event)>,
    pub(crate) event_name: String,
    pub(crate) el: web_sys::Element,
}

impl Drop for EventListener {
    fn drop(&mut self) {
        self.el.remove_event_listener_with_callback(
            &self.event_name,
            &self.inner.as_ref().unchecked_ref(),
        );
    }
}
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::Closure;
use web_sys::Event;
use crate::dom::closure::EventListener;

#[derive(Debug, Clone)]
pub struct Element {
    pub inner: web_sys::Element,
}


fn add_event_listener_boxed(element: &web_sys::Element, event_name: &str, callback: Box<dyn FnMut(web_sys::Event)>) -> Closure<dyn FnMut(Event)> {
    let closure = Closure::wrap(callback);
    element.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref()).unwrap();
    closure
}

impl Element {
    pub fn new(inner: web_sys::Element) -> Self {
        Self { inner }
    }

    /// Use this when you want to add an event listener to an element that you want to later remove.
    /// **NOTE:** If you don't hold on to the returned EventListener, it will be dropped and the event listener will be removed.
    #[must_use]
    pub fn add_removable_event_listener(&self, event_name: &str, callback: impl FnMut(web_sys::Event) + 'static) -> EventListener {
        let closure = add_event_listener_boxed(&self.inner, event_name, Box::new(callback));
        EventListener {
            inner: closure,
            event_name: event_name.to_string(),
            el: self.inner.clone(),
        }
    }

    /// Use this when you want the event listener to live for the lifetime of the element.
    pub fn add_permanent_event_listener(&self, event_name: &str, callback: impl FnMut(web_sys::Event) + 'static) {
        add_event_listener_boxed(&self.inner, event_name, Box::new(callback))
            .forget()
    }

    /// This method exists for consistency with Javascript APIs.
    /// This method requires the lifetime of self, so it's easier just to drop the EventListener directly.
    pub fn remove_event_listener(&self, listener: EventListener) {
        drop(listener);
    }
}
use serde::Deserialize;
use wasm_bindgen::JsValue;

pub struct History {
    inner: web_sys::History,
}

pub fn history() -> History {
    History::global()
}

impl History {
    pub fn global() -> History {
        History {
            inner: web_sys::window().unwrap().history().unwrap(),
        }
    }

    #[inline]
    pub fn back(&self) {
        self.go(-1)
    }

    #[inline]
    pub fn forward(&self) {
        self.go(1)
    }

    pub fn go(&self, delta: i32) {
        self.inner.go_with_delta(delta)
            .expect("history.go should not fail")
    }

    pub fn push<'a, U: Into<&'a str>>(&self, url: U) {
        self.push_state(url.into(), JsValue::undefined())
    }

    pub fn push_state<'a, U: Into<&'a str>, S: Into<JsValue>>(&self, url: U, data: S) {
        self.inner.push_state_with_url(&data.into(), "", Some(&url.into()))
            .expect("history.push_state_with_url should not fail")
    }

    pub fn replace<'a, U: Into<&'a str>>(&self, url: U) {
        self.replace_state(url.into(), JsValue::undefined())
    }

    pub fn replace_state<'a, U: Into<&'a str>, S: Into<JsValue>>(&self, url: U, state: S) {
        self.inner.replace_state_with_url(&state.into(), "", Some(&url.into()))
            .expect("history.replace_state_with_url should not fail")
    }

    pub fn length(&self) -> u32 {
        self.inner.length()
            .expect("history.length should not fail")
    }

    pub fn state<D: for<'a> Deserialize<'a>>(&self) -> serde_json::Result<D> {
        self.inner.state()
            .expect("history.state should not fail")
            .into_serde()
    }
}
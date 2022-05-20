use serde::Deserialize;
pub struct History {}


impl History {
    pub fn new() -> History {
        History {}
    }

    pub fn back(&self) {
        todo!();
    }

    pub fn forward(&self) {
        todo!();
    }

    pub fn go(&self, delta: i32) {
        todo!();
    }

    pub fn push<'a, U: Into<&'a str>>(&self, url: U) {
        todo!();
    }

    pub fn push_state<'a, U: Into<&'a str>, S>(&self, url: U, data: S) {
        todo!();
    }

    pub fn replace<'a, U: Into<&'a str>>(&self, url: U) {
        todo!();
    }

    pub fn replace_state<'a, U: Into<&'a str>>(&self, url: U) {
        todo!();
    }

    pub fn length(&self) -> i32 {
        todo!()
    }

    pub fn state<'a, D: Deserialize<'a>>(&self) -> D {
        todo!()
    }
}
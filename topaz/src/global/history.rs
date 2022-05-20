pub struct History {}


impl History {
    pub fn new() -> History {
        History {}
    }

    pub fn back(&self) {
    }

    pub fn forward(&self) {
    }

    pub fn go(&self, delta: i32) {
    }

    pub fn push<'a, U: Into<&'a str>>(&self, url: U) {
    }

    pub fn push_state<'a, U: Into<&'a str>, S>(&self, url: U, data: S) {
    }

    pub fn replace<'a, U: Into<&'a str>>(&self, url: U) {
    }

    pub fn replace_state<'a, U: Into<&'a str>>(&self, url: U) {
    }

    pub fn length(&self) -> i32 {
        0
    }

    pub fn state(&self) -> &str {
        ""
    }
}
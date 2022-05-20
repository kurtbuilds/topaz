mod document;
mod timer;
mod fetch;
mod window;

pub use fetch::fetch;
pub use document::document;
pub use timer::{set_interval, set_timeout, clear_timeout, clear_interval};
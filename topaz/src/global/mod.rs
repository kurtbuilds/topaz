mod document;
mod timer;
mod fetch;
mod window;
mod history;

pub use fetch::fetch;
pub use document::document;
pub use history::history;
pub use timer::{set_interval, set_timeout, clear_timeout, clear_interval};

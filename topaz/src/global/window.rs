
pub(crate) fn web_sys_window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
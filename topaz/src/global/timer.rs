use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use crate::global::window::web_sys_window;

#[derive(Debug, Clone, Copy)]
pub struct TimeoutId(u32);

#[derive(Debug, Clone, Copy)]
pub struct IntervalId(u32);

///
/// # Examples
///
/// ```
/// let t = topaz::global::set_timeout(|| {
///      println!("Printed after 1s");
/// }, 1000);
/// ```
pub fn set_timeout<F: FnOnce() + 'static>(callback: F, millis: u32) -> TimeoutId {
    let a = Closure::once(Box::new(callback) as Box<dyn FnOnce()>);
    let window = web_sys_window();
    let timeout = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), millis as i32)
        .expect("set_timeout failed");
    a.forget();
    TimeoutId(timeout as u32)
}

pub fn set_interval<F: FnMut() + 'static>(callback: F, timeout: u32) -> IntervalId {
    let a = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
    let window = web_sys_window();
    let timeout = window
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), timeout as i32)
        .expect("set_timeout failed");
    a.forget();
    IntervalId(timeout as u32)
}


pub fn clear_interval(interval_id: IntervalId) {
    web_sys_window().clear_interval_with_handle(interval_id.0 as i32);
}

pub fn clear_timeout(timeout_id: TimeoutId) {
    web_sys_window().clear_timeout_with_handle(timeout_id.0 as i32);
}
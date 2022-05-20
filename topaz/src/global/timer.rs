use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use crate::global::window::web_sys_window;

#[derive(Debug, Clone, Copy)]
pub struct TimeoutId(u32);

#[derive(Debug, Clone, Copy)]
pub struct IntervalId(u32);


/// Splitting this separate from the `set_timeout` function provides a monomorphization benefit
/// The only fn that gets monomorphized is the `set_timeout` function, but it's body is extremely
/// cheap.
fn set_timeout_boxed(callback: Box<dyn FnOnce() + 'static>, millis: u32) -> TimeoutId {
    let a = Closure::once(callback);
    let window = web_sys_window();
    let timeout = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), millis as i32)
        .expect("set_timeout failed");
    a.forget();
    TimeoutId(timeout as u32)
}

///
/// # Examples
///
/// ```
/// let t = topaz::global::set_timeout(|| {
///      println!("Printed after 1s");
/// }, 1000);
/// ```
// TODO measure if inline actually does anything here.
#[inline]
pub fn set_timeout(callback: impl FnOnce() + 'static, millis: u32) -> TimeoutId {
    set_timeout_boxed(Box::new(callback), millis)
}

fn set_interval_boxed(callback: Box<dyn FnMut() + 'static>, timeout: u32) -> IntervalId {
    let a = Closure::wrap(callback);
    let window = web_sys_window();
    let timeout = window
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), timeout as i32)
        .expect("set_timeout failed");
    a.forget();
    IntervalId(timeout as u32)
}

#[inline]
pub fn set_interval(callback: impl FnMut() + 'static, timeout: u32) -> IntervalId {
    set_interval_boxed(Box::new(callback), timeout)
}


pub fn clear_interval(interval_id: IntervalId) {
    web_sys_window().clear_interval_with_handle(interval_id.0 as i32);
}

pub fn clear_timeout(timeout_id: TimeoutId) {
    web_sys_window().clear_timeout_with_handle(timeout_id.0 as i32);
}
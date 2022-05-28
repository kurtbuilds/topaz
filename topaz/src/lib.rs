#![allow(unused)]
#![cfg_attr(feature = "capture-print", feature(internal_output_capture))]
mod observable;
pub mod bind;
pub mod global;
mod dom;
pub mod state2;


#[cfg(feature = "capture-print")]
pub fn hook_println() {
    use std::sync::Arc;
    use std::sync::Mutex;

    let console_buffer = Arc::new(Mutex::new(Vec::new()));
    std::io::set_output_capture(Some(console_buffer.clone()));


    global::set_interval(move || {
        let console_buffer = console_buffer.clone();
        // let idx = idx.clone();
        let mut z = match console_buffer.lock() {
            Ok(z) => z,
            Err(e) => return,
        };
        let s = std::str::from_utf8(&z[..]).unwrap();
        if s.len() > 0 {
            bind::log(s);
            z.clear();
        }
    }, 16);

}

pub fn start() {
    #[cfg(feature = "capture-print")]
    hook_println();
    #[cfg(feature = "capture-panic")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
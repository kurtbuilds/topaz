#![allow(unused)]
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;

use std::io::Read;
use wasm_bindgen::prelude::*;
use topaz::bind::*;
use topaz::global;
use topaz::global::set_timeout;

fn foobar() {
    unimplemented!()
}

#[wasm_bindgen]
pub fn start() {
    topaz::start();

    {
        let mut doc = global::document();
        doc.title = "Topaz".to_string();
    }

    // let mut z = 0;
    //
    // global::set_interval(move || {
    //     z += 1;
    //     println!("Hello, world! {}", z);
    // }, 1000);

    global::set_timeout(move || {
        foobar();
    }, 1000);

    alert(&format!("Hello, {}!", "Foobar"));
}
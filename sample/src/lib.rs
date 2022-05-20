#![feature(box_syntax)]
#![allow(unused)]
#![allow(non_snake_case)]

use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;

use std::io::Read;
use wasm_bindgen::prelude::*;
use topaz::bind::*;
use topaz::global;
use topaz::global::set_timeout;
use percy_dom::{VirtualNode, html, View};
use percy_dom::event::{EventName, MouseEvent};
use percy_dom::PercyDom;
use wasm_bindgen::JsCast;


struct Nav {
}

impl View for Nav {
    fn render(&self) -> VirtualNode {
        html! {
            <nav>
                Branding | Login | Signup
            </nav>
        }
    }
}

fn Nav2() -> VirtualNode {
    html! {
        <div>
        </div>
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}


fn recurse(node: &VirtualNode) {
    match node {
        VirtualNode::Element(e) => {
            if e.events.get(&EventName::new("onclick".into())) != None {
                println!("found event handler {:?}", e.events.get(&EventName::new("onclick".into())));
            }
            for child in e.children.iter() {
                recurse(child.clone());
            }
        },
        VirtualNode::Text(_) => {}
    }
}


#[wasm_bindgen]
pub fn start() {
    topaz::start();

    {
        let mut doc = global::document();
        doc.title = "Topaz".to_string();
    }

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let z = html! {
        <div>
            <Nav/>
            <p>Counter</p>
            <button onclick=move|_event: MouseEvent| {
                web_sys::console::log_1(&"clicked!".into());
            }>
                Click me!
            </button>
        </div>
    };

    let mut clicks = 0;
    let a = move |_| {
        clicks += 1;
        println!("clicked {} times", clicks);
    };

    body.set_inner_html(&z.to_string());

    let doc = global::document();
    let buttons = doc.get_elements_by_tag_name("button");
    let button = buttons.get(0).unwrap().clone();
    let listener = button.add_removable_event_listener("click", a);

    global::set_timeout(move || {
        println!("dropping the event listener");
        button.remove_event_listener(listener);
    }, 5_000);

    println!("buttons: {:?}", buttons);

    alert(&format!("Hello, {}!", "Foobar"));
}
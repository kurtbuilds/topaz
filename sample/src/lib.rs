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
                // println!("clicked");
                web_sys::console::log_1(&"clicked!".into());
            }>
                Click me!
            </button>
        </div>
    };

    let mut clicks = 0;
    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        println!("clicked {} times", clicks);
    }) as Box<dyn FnMut()>);

    recurse(&z);
    body.set_inner_html(&z.to_string());

    let doc = global::document();
    let buttons = doc.get_elements_by_tag_name("button");
    let button = buttons.get(0).unwrap();
    button.inner.add_event_listener_with_callback("click", a.as_ref().unchecked_ref()).unwrap();

    println!("buttons: {:?}", buttons);

    alert(&format!("Hello, {}!", "Foobar"));
    a.forget();
}
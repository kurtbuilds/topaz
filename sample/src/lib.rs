#![feature(box_syntax)]
#![allow(unused)]
#![allow(non_snake_case)]

use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

use std::io::Read;
use std::ops::Deref;
use std::pin::Pin;
use wasm_bindgen::prelude::*;
use topaz::bind::*;
use topaz::global;
use topaz::state;
use topaz::global::set_timeout;
use percy_dom::{VirtualNode, html, View};
use percy_dom::event::{EventName, MouseEvent};
use percy_dom::PercyDom;
use wasm_bindgen::JsCast;
use web_sys::window;
use percy_dom::IterableNodes;


struct Nav {}

impl View for Nav {
    fn render(&self) -> VirtualNode {
        html! {
            <nav>
                Branding | Login | Signup
            </nav>
        }
    }
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
        }
        VirtualNode::Text(_) => {}
    }
}

#[wasm_bindgen]
pub fn start() {
    topaz::start();

    let mut clicks = Rc::new(RefCell::new(0));
    let mut first_render = Rc::new(RefCell::new(true));

    fn render(
        first_render: Rc<RefCell<bool>>,
        clicks: Rc<RefCell<usize>>,
    ) {
        let on_click = {
            let mut clicks = clicks.clone();
            let mut first_render = first_render.clone();
            move |_| {
                let mut clicks_borrow = clicks.borrow_mut();
                // println!("reset clicks: {}", *clicks_borrow);
                *clicks_borrow += 1;
                drop(clicks_borrow);
                render(first_render.clone(), clicks.clone());
            }
        };

        let on_navigate = {
            let mut clicks = clicks.clone();
            let mut first_render = first_render.clone();
            move |_| {
                global::history().push("/login");
                // let mut clicks_borrow = clicks.borrow_mut();
                // // println!("reset clicks: {}", *clicks_borrow);
                // *clicks_borrow += 1;
                // drop(clicks_borrow);
                render(first_render.clone(), clicks.clone());
            }
        };

        let on_google = {
            move |_| {
                global::location().href = "https://google.com".to_string();
            }
        };

        let on_anchor = {
            move |_| {
                use rand::distributions::Alphanumeric;
                use rand::Rng;
                let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect();
                global::location().anchor = s;
            }
        };

        let is_first_render = {
            let mut first_render = first_render.clone();
            let mut first_render_borrow = first_render.borrow_mut();
            let current_value = *first_render_borrow;
            if current_value {
                *first_render_borrow = false;
            }
            current_value
        };

        if (is_first_render) {
            // {
            //     let mut clicks = clicks.clone();
            //     global::set_interval(move || {
            //         let doc = global::document();
            //         let ps = doc.get_elements_by_tag_name("p");
            //         let p = &ps[0];
            //         let mut clicks_borrow = clicks.borrow_mut();
            //         *clicks_borrow += 1;
            //         println!("update: {}", *clicks_borrow);
            //         drop(clicks_borrow);
            //         render(first_render.clone(), clicks.clone());
            //     }, 500)
            // };
        }

        let location = global::location();
        println!("Current path: {}, {:?}", location.path, location.query);

        let vdom = {
            let mut clicks = clicks.borrow_mut();
            html! {
                <div>
                    <Nav/>
                    <p>Counter: {*clicks}</p>
                    <button id="increment" onclick=move|_event: MouseEvent| {
                        web_sys::console::log_1(&"clicked!".into());
                    }>
                        +1
                    </button>
                <button id="navigate">Navigate</button>
                <button id="lmgtfy">LMGTFY</button>
                <button id="anchor">Random Anchor</button>
                </div>
            }
        };

        let body = global::document().body();
        body.set_inner_html(&vdom.to_string());
        let doc = global::document();
        let button = doc.get_element_by_id("increment").unwrap();
        button.add_permanent_event_listener("click", on_click);

        let button = doc.get_element_by_id("navigate").unwrap();
        button.add_permanent_event_listener("click", on_navigate);

        let button = doc.get_element_by_id("lmgtfy").unwrap();
        button.add_permanent_event_listener("click", on_google);

        let button = doc.get_element_by_id("anchor").unwrap();
        button.add_permanent_event_listener("click", on_anchor);
    }

    render(first_render, clicks);
}
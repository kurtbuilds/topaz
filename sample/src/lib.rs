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



// fn Counter() -> VirtualNode {
//     let counter = state!(0);
//     html! {
//         <p> Counter: {counter} </p>
//         <button onclick=|_| {
//             counter += 1;
//         }>
//             +1
//         </button>
//     }
// }

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

// pub struct FakeAtomicUsize {
//     pub value: Pin<*mut usize>
// }
//
// unsafe impl Send for FakeAtomicUsize {}
// unsafe impl Sync for FakeAtomicUsize {}
//
// impl Clone for FakeAtomicUsize {
//     fn clone(&self) -> Self {
//         let ptr: *mut usize = &**self as *const _ as *mut _;
//         FakeAtomicUsize {
//             value: unsafe { Pin::new(&mut *ptr) }
//         }
//     }
// }
//
// impl std::ops::Deref for FakeAtomicUsize {
//     type Target = usize;
//     fn deref(&self) -> &Self::Target {
//         self.value.deref()
//     }
// }
//
// impl std::ops::DerefMut for FakeAtomicUsize {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.value.deref_mut()
//     }
// }
//
// impl std::fmt::Display for FakeAtomicUsize {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", *self.value)
//     }
// }
//
// impl std::fmt::Pointer for FakeAtomicUsize {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:p}", &**self.value.deref())
//     }
// }
//
// impl Drop for FakeAtomicUsize {
//     fn drop(&mut self) {
//         std::mem::forget(self);
//     }
// }

#[wasm_bindgen]
pub fn start() {
    topaz::start();

    let mut clicks = Rc::new(RefCell::new(0));
    let mut first = Rc::new(RefCell::new(true));

    fn render(
        first: Rc<RefCell<bool>>,
        clicks: Rc<RefCell<usize>>,
    ) {
        let on_click = {
            let mut clicks = clicks.clone();
            let mut first = first.clone();
            move |_| {
                let mut z = clicks.borrow_mut();
                println!("reset clicks: {}", *z);
                *z = 0;
                drop(z);
                render(first.clone(), clicks.clone());
            }
        };

        let is_first = {
            let mut first = first.clone();
            let mut is_first = first.borrow_mut();
            let cur = *is_first;
            if cur {
                *is_first = false;
            }
            cur
        };
        if (is_first) {
            {
                let mut clicks = clicks.clone();
                global::set_interval(move || {
                    let doc = global::document();
                    let ps = doc.get_elements_by_tag_name("p");
                    let p = &ps[0];
                    let mut z = clicks.borrow_mut();
                    *z += 1;
                    println!("update: {}", *z);
                    drop(z);
                    render(first.clone(), clicks.clone());
                }, 500)
            };

        }


        let z = {
            let mut clicks = clicks.borrow_mut();
            html! {
                <div>
                    <Nav/>
                    <p>Counter: {*clicks}</p>
                    <button onclick=move|_event: MouseEvent| {
                        web_sys::console::log_1(&"clicked!".into());
                    }>
                        +1
                    </button>
                </div>
            }
        };

        let body = window().unwrap().document().unwrap().body().unwrap();
        body.set_inner_html(&z.to_string());
        let doc = global::document();
        let buttons = doc.get_elements_by_tag_name("button");
        let button = buttons.get(0).unwrap().clone();
        let on_click = button.add_permanent_event_listener("click", on_click);
    };

    render(first, clicks);
}
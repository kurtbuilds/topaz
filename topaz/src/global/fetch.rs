use std::fmt::{Display, Formatter};
use http::{Request, Response};
use web_sys::{RequestInit, RequestMode};
use crate::global::window::web_sys_window;

pub struct FetchError {
    pub message: String,
}

impl std::fmt::Debug for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FetchError: {}", self.message)
    }
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FetchError: {}", self.message)
    }
}

impl std::error::Error for FetchError {
}

/// Inspiration: https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub async fn fetch<T>(request: Request<T>) -> Result<Response<T>, FetchError> {
    // let mut opts = RequestInit::new();
    // opts.method(request.method().as_str());
    // opts.mode(RequestMode::Cors);
    //
    // let url = request.url().as_str();
    // let fetch = web_sys::Request::new_with_str_and_init(&url, &opts)?;
    //
    // request.headers().iter().for_each(|(key, value)| {
    //     fetch.headers().set(key.as_str(), value.to_str().expect("Failed to convert header value to string"))
    //         .expect("Failed to set header");
    // });
    //
    // let window = web_sys_window();
    // let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
    //     .expect("Failed to make fetch request");
    //
    // let resp: web_sys::Response = resp_value.dyn_into().unwrap();
    //
    // let json = resp.json().expect("Failed to ask for response as json")
    //     .await
    //     .expect("Failed to parse json");


    todo!();
}
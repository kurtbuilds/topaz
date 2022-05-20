use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

impl<T: Sized> State<T> {
    pub fn new(value: T) -> State<T> {
        State {
            inner: std::sync::Arc::new(std::sync::Mutex::new(value)),
        }
    }
}

pub struct State<T: Sized> {
    pub inner: std::sync::Arc<std::sync::Mutex<T>>,
}


// impl<'a, T: Sized> Deref for State<T> {
//     type Target = MutexGuard<'a, T>;
//
//     fn deref(&self) -> &Self::Target {
//         self.inner.lock().unwrap().deref()
//     }
// }
//
// impl<T: Sized> std::ops::DerefMut for State<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self.inner.lock().unwrap().deref_mut()
//     }
// }


#[macro_export]
macro_rules! state {
    ($value:expr) => {
        topaz::state2::State::new($value)
    }
}
pub use state;

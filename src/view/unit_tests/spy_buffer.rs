use crate::interfaces::Buffer;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;


static mut CLEAR_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug)]
pub struct SpyBuffer {
    clear_call_count: &'static AtomicUsize,
}

impl SpyBuffer {
    pub fn new() -> Self {
        #[allow(unsafe_code)]
        Self { clear_call_count: unsafe { &CLEAR_CALL_COUNT } }
    }

    #[must_use]
    pub fn count_clear_calls(&self) -> usize {
        self.clear_call_count.load(Ordering::Relaxed)
    }
}

impl Buffer for SpyBuffer {
    fn clear(&mut self) {
        self.clear_call_count.fetch_add(1, Ordering::Relaxed);
    }
}

// // A simpler way?
//
// static CLEAR_CALL_COUNT: Mutex<usize> = Mutex::new(0); // use `lazy_static` crate to resolve
//
// #[derive(Clone, Debug)]
// pub struct SpyBuffer {
//     clear_call_count: &'static Mutex<usize>,
// }
//
// impl SpyBuffer {
//     pub fn new() -> Self {
//         Self { clear_call_count: &CLEAR_CALL_COUNT }
//     }
//
//     #[must_use]
//     pub fn count_clear_calls(&self) -> usize {
//         *self.clear_call_count.lock().unwrap()
//     }
// }
//
// impl Buffer for SpyBuffer {
//     fn clear(&mut self) {
//         self.clear_call_count.lock().unwrap().checked_add(1).unwrap();
//     }
// }

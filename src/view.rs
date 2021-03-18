#[cfg(test)]
mod unit_tests;

use crate::{
    Terminal,
    interfaces::Buffer,
    consts::SIZE
};


// GOOD
#[derive(Debug)]
pub struct View<B> {
    buffer: B,
}

impl<B: Buffer> View<B> {
    pub fn new(buffer: B) -> Self {
        Self {
            buffer,
        }
    }
}

impl<B: Buffer> Terminal for View<B> {
    fn clear(&mut self) {
        self.buffer.clear();
    }
}

// // BAD
// #[derive(Debug)]
// pub struct View {
//     buffer: [[u8; SIZE]; SIZE],
// }
//
// impl View {
//     pub const fn new() -> Self {
//         Self {
//             buffer: [[42; SIZE]; SIZE]
//         }
//     }
// }
//
// impl Terminal for View {
//     fn clear(&mut self) {
//         self.buffer = [[1; SIZE]; SIZE];
//     }
// }

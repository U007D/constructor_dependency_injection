#![allow(non_snake_case)]
mod spy_buffer;

use assert2::assert;
use spy_buffer::SpyBuffer;

use super::*;

#[test]
fn clear__calls_clear_method_on_buffer_trait() {
    // Given
    let buffer = SpyBuffer::new();
    let mut sut = View::new(buffer.clone());

    // When
    sut.clear();

    // Then
    assert!(buffer.count_clear_calls() == 1);
}

// #[test]
// fn clear__clears_injected_buffer_bad() {
//     // Given
//     let sut = View::new;
//
//     // When
//     let res = sut();
//
//     // Then
//     assert!(res.buffer.is_empty())
// }

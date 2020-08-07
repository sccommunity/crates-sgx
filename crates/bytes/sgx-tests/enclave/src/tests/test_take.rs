#![warn(rust_2018_idioms)]

use bytes::buf::{Buf, BufExt};
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn long_take() {
    // Tests that get a take with a size greater than the buffer length will not
    // overrun the buffer. Regression test for #138.
    let buf = b"hello world".take(100);
    assert_eq!(11, buf.remaining());
    assert_eq!(b"hello world", buf.bytes());
}

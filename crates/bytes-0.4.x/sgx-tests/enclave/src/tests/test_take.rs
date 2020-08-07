extern crate bytes;

use bytes::Buf;
use std::io::Cursor;
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn long_take() {
    // Tests that take with a size greater than the buffer length will not
    // overrun the buffer. Regression test for #138.
    let buf = Cursor::new(b"hello world").take(100);
    assert_eq!(11, buf.remaining());
    assert_eq!(b"hello world", buf.bytes());
}

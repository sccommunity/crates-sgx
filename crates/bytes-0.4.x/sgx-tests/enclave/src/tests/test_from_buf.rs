extern crate bytes;

use bytes::{Buf, Bytes, BytesMut};
use std::io::Cursor;
use crates_unittest::test_case;
use std::prelude::v1::*;
const LONG: &'static [u8] = b"mary had a little lamb, little lamb, little lamb";
const SHORT: &'static [u8] = b"hello world";

#[test_case]
fn collect_to_vec() {
    let buf: Vec<u8> = Cursor::new(SHORT).collect();
    assert_eq!(buf, SHORT);

    let buf: Vec<u8> = Cursor::new(LONG).collect();
    assert_eq!(buf, LONG);
}

#[test_case]
fn collect_to_bytes() {
    let buf: Bytes = Cursor::new(SHORT).collect();
    assert_eq!(buf, SHORT);

    let buf: Bytes = Cursor::new(LONG).collect();
    assert_eq!(buf, LONG);
}

#[test_case]
fn collect_to_bytes_mut() {
    let buf: BytesMut = Cursor::new(SHORT).collect();
    assert_eq!(buf, SHORT);

    let buf: BytesMut = Cursor::new(LONG).collect();
    assert_eq!(buf, LONG);
}

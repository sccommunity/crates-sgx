extern crate bytes;

use std::io::{BufRead, Cursor, Read};

use bytes::Buf;
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn read() {
    let buf1 = Cursor::new(b"hello ");
    let buf2 = Cursor::new(b"world");
    let buf = Buf::chain(buf1, buf2); // Disambiguate with Read::chain
    let mut buffer = Vec::new();
    buf.reader().read_to_end(&mut buffer).unwrap();
    assert_eq!(b"hello world", &buffer[..]);
}

#[test_case]
fn buf_read() {
    let buf1 = Cursor::new(b"hell");
    let buf2 = Cursor::new(b"o\nworld");
    let mut reader = Buf::chain(buf1, buf2).reader();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert_eq!("hello\n", &line);
    line.clear();
    reader.read_line(&mut line).unwrap();
    assert_eq!("world", &line);
}

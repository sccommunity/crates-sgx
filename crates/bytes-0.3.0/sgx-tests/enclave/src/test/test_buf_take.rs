use bytes::*;
use std::io::{Cursor, Read};
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
pub fn test_take_from_buf() {
    let mut buf = Take::new(Cursor::new(b"hello world".to_vec()), 5);
    let mut res = vec![];

    buf.read_to_end(&mut res);

    assert_eq!(&res, b"hello");
}

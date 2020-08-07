extern crate bytes;
extern crate futures;
extern crate tokio_buf;

use futures::Async::*;
use std::io::Cursor;
use tokio_buf::{util, BufStream};

use crates_unittest::test_case;
use std::prelude::v1::*;

use crate::buffer_tests::support::*;
type Buf = Cursor<&'static [u8]>;

#[test_case]
fn empty_iter() {
    let mut bs = util::iter(Vec::<Buf>::new());
    assert_none!(bs.poll_buf());
}

#[test_case]
fn full_iter() {
    let bufs = vec![buf(b"one"), buf(b"two"), buf(b"three")];

    let mut bs = util::iter(bufs);
    assert_buf_eq!(bs.poll_buf(), "one");
    assert_buf_eq!(bs.poll_buf(), "two");
    assert_buf_eq!(bs.poll_buf(), "three");
    assert_none!(bs.poll_buf());
}

fn buf(data: &'static [u8]) -> Buf {
    Cursor::new(data)
}

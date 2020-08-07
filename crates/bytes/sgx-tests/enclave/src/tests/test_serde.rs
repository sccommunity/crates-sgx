#![cfg(feature = "serde")]
#![warn(rust_2018_idioms)]

use serde_test::{assert_tokens, Token};
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn test_ser_de_empty() {
    let b = bytes::Bytes::new();
    assert_tokens(&b, &[Token::Bytes(b"")]);
    let b = bytes::BytesMut::with_capacity(0);
    assert_tokens(&b, &[Token::Bytes(b"")]);
}

#[test_case]
fn test_ser_de() {
    let b = bytes::Bytes::from(&b"bytes"[..]);
    assert_tokens(&b, &[Token::Bytes(b"bytes")]);
    let b = bytes::BytesMut::from(&b"bytes"[..]);
    assert_tokens(&b, &[Token::Bytes(b"bytes")]);
}

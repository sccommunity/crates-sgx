use serde_bytes::{ByteBuf, Bytes};
use std::prelude::v1::*;
use crates_unittest::test_case;
fn _bytes_eq_slice(bytes: &Bytes, slice: &[u8]) -> bool {
    bytes == slice
}

fn _bytebuf_eq_vec(bytebuf: ByteBuf, vec: Vec<u8>) -> bool {
    bytebuf == vec
}

fn _bytes_eq_bytestring(bytes: &Bytes) -> bool {
    bytes == b"..."
}

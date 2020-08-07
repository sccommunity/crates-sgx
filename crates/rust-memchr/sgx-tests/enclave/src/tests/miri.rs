// Simple tests using MIRI

use memchr::{memchr, memchr2, memchr3, memrchr, memrchr2, memrchr3};
use std::prelude::v1::*;
use crates_unittest::test_case;
#[test_case]
fn test_with_miri() {
    assert_eq!(memchr(b'a', b"abcda"), Some(0));
    assert_eq!(memchr(b'z', b"abcda"), None);
    assert_eq!(memchr2(b'a', b'z', b"abcda"), Some(0));
    assert_eq!(memchr2(b'z', b'y', b"abcda"), None);
    assert_eq!(memchr3(b'a', b'z', b'b', b"abcda"), Some(0));
    assert_eq!(memchr3(b'z', b'y', b'x', b"abcda"), None);
    assert_eq!(memrchr(b'a', b"abcda"), Some(4));
    assert_eq!(memrchr(b'z', b"abcda"), None);
    assert_eq!(memrchr2(b'a', b'z', b"abcda"), Some(4));
    assert_eq!(memrchr2(b'z', b'y', b"abcda"), None);
    assert_eq!(memrchr3(b'a', b'z', b'b', b"abcda"), Some(4));
    assert_eq!(memrchr3(b'z', b'y', b'x', b"abcda"), None);
}

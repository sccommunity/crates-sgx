// use memchr::fallback;
// use memchr::naive;
use memchr::{memchr, memchr2, memchr3, memrchr, memrchr2, memrchr3};

use crate::tests::memchr_tests;
use std::prelude::v1::*;
use crates_unittest::test_case;
#[test_case]
fn memchr1_find() {
    for test in memchr_tests() {
        test.one(false, memchr);
    }
}

// #[test_case]
// fn memchr1_fallback_find() {
//     for test in memchr_tests() {
//         test.one(false, fallback::memchr);
//     }
// }

#[test_case]
fn memchr2_find() {
    for test in memchr_tests() {
        test.two(false, memchr2);
    }
}

// #[test_case]
// fn memchr2_fallback_find() {
//     for test in memchr_tests() {
//         test.two(false, fallback::memchr2);
//     }
// }

#[test_case]
fn memchr3_find() {
    for test in memchr_tests() {
        test.three(false, memchr3);
    }
}

// #[test_case]
// fn memchr3_fallback_find() {
//     for test in memchr_tests() {
//         test.three(false, fallback::memchr3);
//     }
// }

#[test_case]
fn memrchr1_find() {
    for test in memchr_tests() {
        test.one(true, memrchr);
    }
}

// #[test_case]
// fn memrchr1_fallback_find() {
//     for test in memchr_tests() {
//         test.one(true, fallback::memrchr);
//     }
// }

#[test_case]
fn memrchr2_find() {
    for test in memchr_tests() {
        test.two(true, memrchr2);
    }
}

// #[test_case]
// fn memrchr2_fallback_find() {
//     for test in memchr_tests() {
//         test.two(true, fallback::memrchr2);
//     }
// }

#[test_case]
fn memrchr3_find() {
    for test in memchr_tests() {
        test.three(true, memrchr3);
    }
}

// #[test_case]
// fn memrchr3_fallback_find() {
//     for test in memchr_tests() {
//         test.three(true, fallback::memrchr3);
//     }
// }

quickcheck! {
    fn qc_memchr1_matches_naive(n1: u8, corpus: Vec<u8>) -> bool {
        memchr(n1, &corpus) == naive::memchr(n1, &corpus)
    }
}

quickcheck! {
    fn qc_memchr2_matches_naive(n1: u8, n2: u8, corpus: Vec<u8>) -> bool {
        memchr2(n1, n2, &corpus) == naive::memchr2(n1, n2, &corpus)
    }
}

quickcheck! {
    fn qc_memchr3_matches_naive(
        n1: u8, n2: u8, n3: u8,
        corpus: Vec<u8>
    ) -> bool {
        memchr3(n1, n2, n3, &corpus) == naive::memchr3(n1, n2, n3, &corpus)
    }
}

quickcheck! {
    fn qc_memrchr1_matches_naive(n1: u8, corpus: Vec<u8>) -> bool {
        memrchr(n1, &corpus) == naive::memrchr(n1, &corpus)
    }
}

quickcheck! {
    fn qc_memrchr2_matches_naive(n1: u8, n2: u8, corpus: Vec<u8>) -> bool {
        memrchr2(n1, n2, &corpus) == naive::memrchr2(n1, n2, &corpus)
    }
}

quickcheck! {
    fn qc_memrchr3_matches_naive(
        n1: u8, n2: u8, n3: u8,
        corpus: Vec<u8>
    ) -> bool {
        memrchr3(n1, n2, n3, &corpus) == naive::memrchr3(n1, n2, n3, &corpus)
    }
}

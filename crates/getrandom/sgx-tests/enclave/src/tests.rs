use std::prelude::v1::*;
use std::mem::size_of;
use getrandom::Error;
use getrandom::getrandom;

pub fn test_size() {
    assert_eq!(size_of::<Error>(), 4);
    assert_eq!(size_of::<Result<(), Error>>(), 4);
}

pub fn test_zero() {
    // Test that APIs are happy with zero-length requests
    getrandom(&mut [0u8; 0]).unwrap();
}

pub fn test_diff() {
    let mut v1 = [0u8; 1000];
    getrandom(&mut v1).unwrap();

    let mut v2 = [0u8; 1000];
    getrandom(&mut v2).unwrap();

    let mut n_diff_bits = 0;
    for i in 0..v1.len() {
        n_diff_bits += (v1[i] ^ v2[i]).count_ones();
    }

    // Check at least 1 bit per byte differs. p(failure) < 1e-1000 with random input.
    assert!(n_diff_bits >= v1.len() as u32);
}

pub fn test_huge() {
    let mut huge = [0u8; 100_000];
    getrandom(&mut huge).unwrap();
}

pub fn run_tests() {
    use sgx_tunittest::*;
    rsgx_unit_tests!(
        test_size,
        test_huge,
        test_zero,
        test_diff
    );
}

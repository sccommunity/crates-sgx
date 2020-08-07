#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
extern crate sgx_tstd as std;

use instant::Instant;
use std::time::Duration;
use std::prelude::v1::*;
use crates_unittest::{ run_inventory_tests, test_case };

#[test_case]
fn test_instant_now() {
    let now = Instant::now();
    assert!(now.elapsed().as_nanos() > 0);
}
#[test_case]
fn test_duration() {
    let now = Instant::now();
    let one_sec = Duration::from_secs(1);
    assert!(now.elapsed() < one_sec);
}

// Duration::new will overflow when you have u64::MAX seconds and one billion nanoseconds.
// <https://doc.rust-lang.org/std/time/struct.Duration.html#method.new>
const ONE_BILLION: u32 = 1_000_000_000;
#[test_case]
fn test_checked_add() {
    let now = Instant::now();

    assert!(now.checked_add(Duration::from_millis(1)).is_some());
    assert_eq!(
        None,
        now.checked_add(Duration::new(u64::MAX, ONE_BILLION - 1))
    );
}
#[test_case]
fn test_checked_sub() {
    let now = Instant::now();

    assert!(now.checked_sub(Duration::from_millis(1)).is_some());
    assert!(now
        .checked_sub(Duration::new(u64::MAX, ONE_BILLION - 1))
        .is_none());
}

#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    run_inventory_tests!();
}

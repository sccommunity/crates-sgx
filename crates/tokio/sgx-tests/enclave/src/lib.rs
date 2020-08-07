#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
#[macro_use]
extern crate pin_project_lite;
use std::prelude::v1::*;


mod tests;
mod tokio_test;
mod tokio_util;
use tests::run_tests;

#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    run_tests();
}

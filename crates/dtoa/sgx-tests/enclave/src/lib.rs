#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
#[macro_use]
extern crate dtoa;
use std::prelude::v1::*;
use crates_unittest::run_inventory_tests;

mod test;

#[no_mangle]
pub extern "C" fn ecall_run_tests() {
   
    run_inventory_tests!();
}

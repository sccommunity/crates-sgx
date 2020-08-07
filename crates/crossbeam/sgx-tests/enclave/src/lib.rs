#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
#[macro_use]
extern crate crossbeam_channel;
extern crate crossbeam_utils;
extern crate crossbeam_deque;
extern crate crossbeam_epoch;
extern crate crossbeam_queue;
extern crate crossbeam_skiplist;
extern crate rand;

use std::prelude::v1::*;
use crates_unittest::run_inventory_tests;

mod channel_test;
mod deque_test;
mod queue_test;
mod skiplist_test;
mod utils_test;


#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    run_inventory_tests!();
}

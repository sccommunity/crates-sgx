#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

#[macro_use]
extern crate tokio_io;
use std::prelude::v1::*;
use crates_unittest::run_inventory_tests;

mod tokio_tests;
mod buffer_tests;
 mod codec_tests;
mod current_thread_tests;
mod executor_tests;
mod fs_tests;
mod io_tests;
mod signal_tests;
mod sync_tests;
mod tcp_tests;
mod threadpool_tests;
mod timer_tests;

mod udp_tests;
mod uds_tests;

#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    run_inventory_tests!();
}

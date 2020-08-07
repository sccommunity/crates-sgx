#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate tokio;
extern crate lazy_static;
extern crate futures_util;
extern crate crates_unittest;
extern crate tokio_rustls;
mod tests;


#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    tests::run_tests();
}

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

#[macro_use]
extern crate sval;

#[macro_use]
extern crate serde;

mod serde_alloc;

#[no_mangle]
pub extern "C" fn ocall_run_tests() {
    serde_alloc::run_tests();
}

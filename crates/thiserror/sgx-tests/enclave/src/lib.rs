#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
extern crate sgx_tstd as std;

mod test_display;

#[no_mangle]
pub extern "C" fn ocall_run_tests() {
    test_display::run_tests();
}

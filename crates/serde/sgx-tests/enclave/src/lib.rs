#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

mod test_annotations;
mod test_value;
mod test_ser;
#[macro_use]
mod macros;

#[no_mangle]
pub extern "C" fn ocall_run_tests() {
    test_annotations::run_tests();
    test_value::run_tests();
    test_ser::run_tests();
}

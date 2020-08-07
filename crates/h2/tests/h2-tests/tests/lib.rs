
//#![feature(custom_test_frameworks)]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]


mod client_request;
mod codec_read;
mod codec_write;
mod flow_control;
mod hammer;
mod ping_pong;
mod prioritization;
mod push_promise;
mod server;
mod stream_states;
mod trailers;



#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(feature = "enclave_unit_test")]
pub mod tests {
    use std::prelude::v1::*;
    use std::string::ToString;
    use crates_unittest::{ run_inventory_tests };
   
    pub fn run_tests() {
        run_inventory_tests!();
    }
}
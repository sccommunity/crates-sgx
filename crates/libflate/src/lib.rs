//! A Rust implementation of DEFLATE algorithm and related formats (ZLIB, GZIP).
#![warn(missing_docs)]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

pub use finish::Finish;


macro_rules! invalid_data_error {
    ($fmt:expr) => { invalid_data_error!("{}", $fmt) };
    ($fmt:expr, $($arg:tt)*) => {
        ::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!($fmt, $($arg)*))
    }
}

macro_rules! finish_try {
    ($e:expr) => {
        match $e.unwrap() {
            (inner, None) => inner,
            (inner, error) => return crate::finish::Finish::new(inner, error),
        }
    };
}

pub mod deflate;
pub mod finish;
pub mod gzip;
pub mod lz77;
pub mod non_blocking;
pub mod zlib;

mod bit;
mod checksum;
mod huffman;
mod util;

#[cfg(feature = "enclave_unit_test")]
pub mod tests {
    use std::prelude::v1::*;
    use crates_unittest::run_inventory_tests;

    pub fn run_tests() {
        run_inventory_tests!();
    }
}

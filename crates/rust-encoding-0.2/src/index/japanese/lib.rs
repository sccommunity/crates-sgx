// This is a part of rust-encoding.
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

//! Japanese index tables for [rust-encoding](https://github.com/lifthrasiir/rust-encoding).

#![cfg_attr(test, feature(test))]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate sgx_tstd as std;
#[cfg(feature = "enclave_unit_test")]
#[macro_use]
extern crate encoding_index_tests;

/// JIS X 0208 with common extensions.
///
/// From the Encoding Standard:
///
/// > This is the JIS X 0208 standard including formerly proprietary extensions from IBM and NEC.
pub mod jis0208;

/// JIS X 0212.
///
/// From the Encoding Standard:
///
/// > This is the JIS X 0212 standard.
/// > It is only used by the euc-jp decoder due to lack of widespread support elsewhere.
pub mod jis0212;


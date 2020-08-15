#![doc(html_root_url = "https://docs.rs/tower-limit/0.3.1")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![allow(elided_lifetimes_in_paths)]

//! Tower middleware for limiting requests.
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;
pub mod concurrency;
pub mod rate;

pub use crate::{
    concurrency::{ConcurrencyLimit, ConcurrencyLimitLayer},
    rate::{RateLimit, RateLimitLayer},
};

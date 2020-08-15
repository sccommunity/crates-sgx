//! A cache of services

#![doc(html_root_url = "https://docs.rs/tower-ready-cache/0.3.1")]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;
pub mod cache;
pub mod error;

pub use self::cache::ReadyCache;

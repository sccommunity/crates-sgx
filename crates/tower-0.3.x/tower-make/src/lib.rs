#![doc(html_root_url = "https://docs.rs/tower-make/0.3.0")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Trait aliases for Services that produce specific types of Responses.
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate sgx_tstd as std;

#[cfg(feature = "connect")]
mod make_connection;
mod make_service;

#[cfg(feature = "connect")]
pub use crate::make_connection::MakeConnection;
pub use crate::make_service::MakeService;
use std::prelude::v1::*;
mod sealed {
    pub trait Sealed<T> {}
}

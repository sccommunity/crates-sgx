#![doc(html_root_url = "https://docs.rs/tower-spawn-ready/0.3.0")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![allow(elided_lifetimes_in_paths)]

//! When an underlying service is not ready, drive it to readiness on a
//! background task.
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

use std::prelude::v1::*;
pub mod future;
mod layer;
mod make;
mod service;

pub use crate::layer::SpawnReadyLayer;
pub use crate::make::{MakeFuture, MakeSpawnReady};
pub use crate::service::SpawnReady;

/// Errors produced by `SpawnReady`.
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

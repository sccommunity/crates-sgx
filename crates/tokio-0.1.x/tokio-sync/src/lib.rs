#![doc(html_root_url = "https://docs.rs/tokio-sync/0.1.8")]
#![deny(missing_debug_implementations, missing_docs, unreachable_pub)]

//! Asynchronous synchronization primitives.
//!
//! > **Note:** This crate is **deprecated in tokio 0.2.x** and has been moved into
//! > [`tokio::sync`] behind the `sync` [feature flag].
//!
//! [`tokio::sync`]: https://docs.rs/tokio/latest/tokio/sync/index.html
//! [feature flag]: https://docs.rs/tokio/latest/tokio/index.html#feature-flags
//!
//! This crate provides primitives for synchronizing asynchronous tasks.

#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate fnv;
#[macro_use]
extern crate futures;

macro_rules! debug {
    ($($t:tt)*) => {
        if false {
            println!($($t)*);
        }
    }
}

macro_rules! if_fuzz {
    ($($t:tt)*) => {{
        if false { $($t)* }
    }}
}

pub mod lock;
mod loom;
pub mod mpsc;
pub mod oneshot;
pub mod semaphore;
pub mod task;
pub mod watch;

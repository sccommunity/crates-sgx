//! Core traits and types for asynchronous operations in Rust.

#![cfg_attr(feature = "cfg-target-has-atomic", feature(cfg_target_has_atomic))]
#![cfg_attr(any(all(feature = "mesalock_sgx",
                    not(target_env = "sgx")),
                not(feature = "std")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
// It cannot be included in the published code because this lints have false positives in the minimum required version.
#![cfg_attr(test, warn(single_use_lifetimes))]
#![warn(clippy::all)]

#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]

#![doc(html_root_url = "https://docs.rs/futures-core/0.3.5")]

#[cfg(all(feature = "cfg-target-has-atomic", not(feature = "unstable")))]
compile_error!("The `cfg-target-has-atomic` feature requires the `unstable` feature as an explicit opt-in to unstable features");

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod future;
#[doc(hidden)] pub use self::future::{Future, FusedFuture, TryFuture};

pub mod stream;
#[doc(hidden)] pub use self::stream::{Stream, FusedStream, TryStream};

#[macro_use]
pub mod task;

// Not public API.
#[doc(hidden)]
pub mod core_reexport {
    #[doc(hidden)]
    pub use core::*;
}

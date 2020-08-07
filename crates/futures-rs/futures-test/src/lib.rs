//! Utilities to make testing [`Future`s](futures_core::future::Future) easier

#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
// It cannot be included in the published code because this lints have false positives in the minimum required version.
#![cfg_attr(test, warn(single_use_lifetimes))]
#![warn(clippy::all)]

#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]

#![doc(html_root_url = "https://docs.rs/futures-test/0.3.5")]

#![cfg_attr(any(all(feature = "mesalock_sgx",
                    not(target_env = "sgx")),
                not(feature = "std")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(not(feature = "std"))]
compile_error!("`futures-test` must have the `std` feature activated, this is a default-active feature");

#[doc(hidden)]
#[cfg(feature = "std")]
pub use sgx_tstd as std_reexport;

#[doc(hidden)]
#[cfg(feature = "std")]
pub extern crate futures_core as futures_core_reexport;

#[macro_use]
#[doc(hidden)]
#[cfg(feature = "std")]
pub mod assert;

#[cfg(feature = "std")]
pub mod task;

#[cfg(feature = "std")]
pub mod future;

#[cfg(feature = "std")]
pub mod stream;

#[cfg(feature = "std")]
pub mod io;

mod interleave_pending;

#![doc(html_root_url = "https://docs.rs/tokio-test/0.2.1")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(intra_doc_link_resolution_failure)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

//! Tokio and Futures based testing utilites
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

pub mod io;

mod macros;
pub mod task;

/// Runs the provided future, blocking the current thread until the
/// future completes.
///
/// For more information, see the documentation for
/// [`tokio::runtime::current_thread::Runtime::block_on`][runtime-block-on].
///
/// [runtime-block-on]: https://docs.rs/tokio/0.2.0-alpha.2/tokio/runtime/current_thread/struct.Runtime.html#method.block_on
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use tokio::runtime;

    let mut rt = runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(future)
}

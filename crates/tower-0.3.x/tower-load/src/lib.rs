//! Abstractions and utilties for measuring a service's load.

#![doc(html_root_url = "https://docs.rs/tower-load/0.3.0")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![allow(elided_lifetimes_in_paths)]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;
mod constant;
mod instrument;
pub mod peak_ewma;
pub mod pending_requests;

pub use self::{
    constant::Constant,
    instrument::{Instrument, InstrumentFuture, NoInstrument},
    peak_ewma::{PeakEwma, PeakEwmaDiscover},
    pending_requests::{PendingRequests, PendingRequestsDiscover},
};

/// Exposes a load metric.
pub trait Load {
    /// A comparable load metric. Lesser values are "preferable" to greater values.
    type Metric: PartialOrd;

    /// Obtains a service's load.
    fn load(&self) -> Self::Metric;
}

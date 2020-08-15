//! A `tonic` based gRPC healthcheck implementation.
//!
//! # Example
//!
//! An example can be found [here].
//!
//! [here]: https://github.com/hyperium/tonic/blob/master/examples/src/health/server.rs
#![feature(proc_macro_hygiene)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(
    html_logo_url = "https://github.com/hyperium/tonic/raw/master/.github/assets/tonic-docs.png"
)]
#![doc(html_root_url = "https://docs.rs/tonic/0.2.0")]
#![doc(issue_tracker_base_url = "https://github.com/hyperium/tonic/issues/")]
#![doc(test(no_crate_inject, attr(deny(rust_2018_idioms))))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;
use std::fmt::{Display, Formatter};

mod proto {
    #![allow(unreachable_pub)]
    tonic::include_proto!("grpc.health.v1");
}

pub mod server;

/// An enumeration of values representing gRPC service health.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ServingStatus {
    /// Unknown status
    Unknown,
    /// The service is currently up and serving requests.
    Serving,
    /// The service is currently down and not serving requests.
    NotServing,
}

impl Display for ServingStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServingStatus::Unknown => f.write_str("Unknown"),
            ServingStatus::Serving => f.write_str("Serving"),
            ServingStatus::NotServing => f.write_str("NotServing"),
        }
    }
}

impl From<ServingStatus> for proto::health_check_response::ServingStatus {
    fn from(s: ServingStatus) -> Self {
        match s {
            ServingStatus::Unknown => proto::health_check_response::ServingStatus::Unknown,
            ServingStatus::Serving => proto::health_check_response::ServingStatus::Serving,
            ServingStatus::NotServing => proto::health_check_response::ServingStatus::NotServing,
        }
    }
}

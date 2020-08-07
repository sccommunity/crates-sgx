//! Human-friendly time parser and formatter
//!
//! Features:
//!
//! * Parses durations in free form like `15days 2min 2s`
//! * Formats durations in similar form `2years 2min 12us`
//! * Parses and formats timestamp in `rfc3339` format: `2018-01-01T12:53:00Z`
//! * Parses timestamps in a weaker format: `2018-01-01 12:53:00`
//!
//! Timestamp parsing/formatting is super-fast because format is basically
//! fixed.
//!
//! See [humantime-serde] for serde integration (previous crate [serde-humantime] looks unmaintained).
//!
//! [serde-humantime]: https://docs.rs/serde-humantime/0.1.1/serde_humantime/
//! [humantime-serde]: https://docs.rs/humantime-serde

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

mod duration;
mod wrapper;
mod date;

pub use self::duration::{parse_duration, Error as DurationError};
pub use self::duration::{format_duration, FormattedDuration};
pub use self::wrapper::{Duration, Timestamp};
pub use self::date::{parse_rfc3339, parse_rfc3339_weak, Error as TimestampError};
pub use self::date::{
    format_rfc3339, format_rfc3339_micros, format_rfc3339_millis, format_rfc3339_nanos,
    format_rfc3339_seconds,
};
pub use self::date::{Rfc3339Timestamp};

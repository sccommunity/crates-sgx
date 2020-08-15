//! Error types

use std::{error, fmt};
use std::prelude::v1::*;
pub(crate) type Error = Box<dyn error::Error + Send + Sync>;

/// The timeout elapsed.
#[derive(Debug)]
pub struct Elapsed(pub(super) ());

impl Elapsed {
    /// Construct a new elapsed error
    pub fn new() -> Self {
        Elapsed(())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timed out")
    }
}

impl error::Error for Elapsed {}

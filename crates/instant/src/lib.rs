#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate sgx_tstd as std;

#[cfg(all(any(target_arch = "wasm32", target_arch = "asmjs"), feature = "stdweb"))]
#[macro_use]
extern crate stdweb;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
pub use crate::native::Instant;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
pub use crate::wasm::Instant;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
#[cfg(feature = "now")]
pub use crate::native::now;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
#[cfg(feature = "now")]
pub use crate::wasm::now;




pub use std::time::Duration;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
mod native;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
mod wasm;

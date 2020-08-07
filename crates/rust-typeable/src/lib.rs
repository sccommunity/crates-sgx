#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]

//! Exposes `Typeable`, which exposes the `get_type` method, which gives
//! the `TypeId` of any 'static type.
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate sgx_tstd as std;
use std::any::{Any, TypeId};

/// Universal mixin trait for adding a `get_type` method.
///
pub trait Typeable: Any {
    /// Get the `TypeId` of this object.
    #[inline(always)]
    fn get_type(&self) -> TypeId { TypeId::of::<Self>() }
}

impl<T: Any> Typeable for T {}


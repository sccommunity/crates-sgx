//! MIO bindings for Unix Domain Sockets

#![cfg(unix)]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/mio-uds/0.6")]
#![allow(unused_imports)]
#![allow(deprecated)]
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate iovec;
extern crate sgx_libc;
extern crate mio;

use std::io;

mod datagram;
mod listener;
mod socket;
mod stream;

pub use stream::UnixStream;
pub use listener::UnixListener;
pub use datagram::UnixDatagram;

fn cvt(i: sgx_libc::c_int) -> io::Result<sgx_libc::c_int> {
    if i == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}

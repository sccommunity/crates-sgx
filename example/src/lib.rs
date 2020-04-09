#![allow(unused_imports)]
#![cfg_attr(feature = "mesalock_sgx", no_std)]
#[cfg(feature = "mesalock_sgx")]
extern crate sgx_tstd as std;

use lazy_static::lazy_static;
use either;
use ryu;
use log;
use getrandom;

#[macro_use]
extern crate matches;

pub fn example() {}

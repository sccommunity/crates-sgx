#![cfg(unix)]
extern crate sgx_libc as libc;
use crates_unittest::test_case;
use std::prelude::v1::*;
use super::support::*;
#[test_case]
fn simple() {
    let mut rt = CurrentThreadRuntime::new().unwrap();
    let signal =
        run_with_timeout(&mut rt, Signal::new(libc::SIGUSR1)).expect("failed to create signal");

    send_signal(libc::SIGUSR1);

    run_with_timeout(&mut rt, signal.into_future())
        .ok()
        .expect("failed to get signal");
}

#![cfg(unix)]

extern crate sgx_libc as libc;

use super::support::*;
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn tokio_simple() {
    let signal_future = Signal::new(libc::SIGUSR1).and_then(|signal| {
        send_signal(libc::SIGUSR1);
        signal.into_future().map(|_| ()).map_err(|(err, _)| err)
    });

    let mut rt = CurrentThreadRuntime::new().expect("failed to init runtime");
    run_with_timeout(&mut rt, signal_future).expect("failed");
}

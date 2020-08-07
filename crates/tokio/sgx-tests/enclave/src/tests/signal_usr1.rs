#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
#![cfg(unix)]


use crate::tests::support::signal::send_signal;

use tokio::signal::unix::{signal, SignalKind};
use tokio_test::assert_ok;
use std::string::ToString;
use crates_unittest::test_case;
use sgx_libc as libc;

#[crates_unittest::test]
async fn signal_usr1() {
    let mut signal = assert_ok!(
        signal(SignalKind::user_defined1()),
        "failed to create signal"
    );

    send_signal(libc::SIGUSR1);

    signal.recv().await;
}

#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
#![cfg(unix)]


use crate::tests::support::signal::send_signal;

use tokio::signal::unix::{signal, SignalKind};
use std::string::ToString;
use crates_unittest::test_case;
use sgx_libc as libc;

#[crates_unittest::test]
async fn drop_then_get_a_signal() {
    let kind = SignalKind::user_defined1();
    let sig = signal(kind).expect("failed to create first signal");
    drop(sig);

    send_signal(libc::SIGUSR1);
    let mut sig = signal(kind).expect("failed to create second signal");

    let _ = sig.recv().await;
}

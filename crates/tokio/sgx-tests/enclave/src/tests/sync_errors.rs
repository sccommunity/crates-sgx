#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
use std::string::ToString;
use crates_unittest::test_case;
fn is_error<T: std::error::Error + Send + Sync>() {}

#[test_case]
fn mpsc_error_bound() {
    use tokio::sync::mpsc::error;

    is_error::<error::SendError<()>>();
    is_error::<error::TrySendError<()>>();
}

#[test_case]
fn oneshot_error_bound() {
    use tokio::sync::oneshot::error;

    is_error::<error::RecvError>();
    is_error::<error::TryRecvError>();
}

#[test_case]
fn watch_error_bound() {
    use tokio::sync::watch::error;

    is_error::<error::SendError<()>>();
}

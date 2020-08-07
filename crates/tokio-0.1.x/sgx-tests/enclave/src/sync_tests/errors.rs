extern crate tokio_sync;
use crates_unittest::test_case;
use std::prelude::v1::*;
fn is_error<T: ::std::error::Error + Send + Sync>() {}

#[test_case]
fn mpsc_error_bound() {
    use tokio_sync::mpsc::error;

    is_error::<error::RecvError>();
    is_error::<error::SendError>();
    is_error::<error::TrySendError<()>>();
    is_error::<error::UnboundedRecvError>();
    is_error::<error::UnboundedSendError>();
    is_error::<error::UnboundedTrySendError<()>>();
}

#[test_case]
fn oneshot_error_bound() {
    use tokio_sync::oneshot::error;

    is_error::<error::RecvError>();
    is_error::<error::TryRecvError>();
}

#[test_case]
fn watch_error_bound() {
    use tokio_sync::watch::error;

    is_error::<error::RecvError>();
    is_error::<error::SendError<()>>();
}

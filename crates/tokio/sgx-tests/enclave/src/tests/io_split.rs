#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::io::{split, AsyncRead, AsyncWrite, ReadHalf, WriteHalf};

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::string::ToString;
use crates_unittest::test_case;

struct RW;

impl AsyncRead for RW {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(1))
    }
}

impl AsyncWrite for RW {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        Poll::Ready(Ok(1))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}

#[test_case]
fn is_send_and_sync() {
    fn assert_bound<T: Send + Sync>() {}

    assert_bound::<ReadHalf<RW>>();
    assert_bound::<WriteHalf<RW>>();
}

#[test_case]
fn split_stream_id() {
    let (r1, w1) = split(RW);
    let (r2, w2) = split(RW);
    assert_eq!(r1.is_pair_of(&w1), true);
    assert_eq!(r1.is_pair_of(&w2), false);
    assert_eq!(r2.is_pair_of(&w2), true);
    assert_eq!(r2.is_pair_of(&w1), false);
}

#[test_case]
fn unsplit_ok() {
    let (r, w) = split(RW);
    r.unsplit(w);
}

// #[test_case]
// #[should_panic]
fn unsplit_err1() {
    let (r, _) = split(RW);
    let (_, w) = split(RW);
    r.unsplit(w);
}

// #[test_case]
// #[should_panic]
fn unsplit_err2() {
    let (_, w) = split(RW);
    let (r, _) = split(RW);
    r.unsplit(w);
}

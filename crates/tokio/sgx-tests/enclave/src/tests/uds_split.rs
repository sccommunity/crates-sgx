#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
#![cfg(unix)]

use tokio::net::UnixStream;
use tokio::prelude::*;
use std::string::ToString;
use crates_unittest::test_case;
use std::prelude::v1::*;
use tokio_test::assert_ok;
/// Checks that `UnixStream` can be split into a read half and a write half using
/// `UnixStream::split` and `UnixStream::split_mut`.
///
/// Verifies that the implementation of `AsyncWrite::poll_shutdown` shutdowns the stream for
/// writing by reading to the end of stream on the other side of the connection.

#[crates_unittest::test]
async fn split() {
   assert_ok!(split_inner().await);
}

async fn split_inner() -> std::io::Result<()> {
    let (mut a, mut b) = UnixStream::pair()?;

    let (mut a_read, mut a_write) = a.split();
    let (mut b_read, mut b_write) = b.split();

    let (a_response, b_response) = futures::future::try_join(
        send_recv_all(&mut a_read, &mut a_write, b"A"),
        send_recv_all(&mut b_read, &mut b_write, b"B"),
    )
    .await?;

    assert_eq!(a_response, b"B");
    assert_eq!(b_response, b"A");

    Ok(())
}

async fn send_recv_all(
    read: &mut (dyn AsyncRead + Unpin),
    write: &mut (dyn AsyncWrite + Unpin),
    input: &[u8],
) -> std::io::Result<Vec<u8>> {
    write.write_all(input).await?;
    write.shutdown().await?;

    let mut output = Vec::new();
    read.read_to_end(&mut output).await?;
    Ok(output)
}

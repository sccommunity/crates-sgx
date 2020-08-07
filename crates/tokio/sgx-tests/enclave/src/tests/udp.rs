#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::net::UdpSocket;
use std::string::ToString;
use crates_unittest::test_case;
use tokio_test::assert_ok;

const MSG: &[u8] = b"hello";
const MSG_LEN: usize = MSG.len();
#[crates_unittest::test]
async fn send_recv() {
    assert_ok!(send_recv_inner().await);
}
async fn send_recv_inner() -> std::io::Result<()> {
    let mut sender = UdpSocket::bind("127.0.0.1:0").await?;
    let mut receiver = UdpSocket::bind("127.0.0.1:0").await?;

    sender.connect(receiver.local_addr()?).await?;
    receiver.connect(sender.local_addr()?).await?;

    sender.send(MSG).await?;

    let mut recv_buf = [0u8; 32];
    let len = receiver.recv(&mut recv_buf[..]).await?;

    assert_eq!(&recv_buf[..len], MSG);
    Ok(())
}

#[crates_unittest::test]
async fn send_to_recv_from() {
    assert_ok!(send_to_recv_from_inner().await);
}

async fn send_to_recv_from_inner() -> std::io::Result<()> {
    let mut sender = UdpSocket::bind("127.0.0.1:0").await?;
    let mut receiver = UdpSocket::bind("127.0.0.1:0").await?;

    let receiver_addr = receiver.local_addr()?;
    sender.send_to(MSG, &receiver_addr).await?;

    let mut recv_buf = [0u8; 32];
    let (len, addr) = receiver.recv_from(&mut recv_buf[..]).await?;

    assert_eq!(&recv_buf[..len], MSG);
    assert_eq!(addr, sender.local_addr()?);
    Ok(())
}

#[crates_unittest::test]
async fn split() {
    assert_ok!(split_inner().await);
}

async fn split_inner() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let (mut r, mut s) = socket.split();

    let addr = s.as_ref().local_addr()?;
    tokio::spawn(async move {
        s.send_to(MSG, &addr).await.unwrap();
    });
    let mut recv_buf = [0u8; 32];
    let (len, _) = r.recv_from(&mut recv_buf[..]).await?;
    assert_eq!(&recv_buf[..len], MSG);
    Ok(())
}

#[crates_unittest::test]
async fn reunite() {
    assert_ok!(reunite_inner().await);
}

async fn reunite_inner() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let (s, r) = socket.split();
    assert!(s.reunite(r).is_ok());
    Ok(())
}

#[crates_unittest::test]
async fn reunite_error() {
    assert_ok!(reunite_error_inner().await);
}

async fn reunite_error_inner() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let socket1 = UdpSocket::bind("127.0.0.1:0").await?;
    let (s, _) = socket.split();
    let (_, r1) = socket1.split();
    assert!(s.reunite(r1).is_err());
    Ok(())
}

// # Note
//
// This test is purposely written such that each time `sender` sends data on
// the socket, `receiver` awaits the data. On Unix, it would be okay waiting
// until the end of the test to receive all the data. On Windows, this would
// **not** be okay because it's resources are completion based (via IOCP).
// If data is sent and not yet received, attempting to send more data will
// result in `ErrorKind::WouldBlock` until the first operation completes.
#[crates_unittest::test]
async fn try_send_spawn() {
    const MSG2: &[u8] = b"world!";
    const MSG2_LEN: usize = MSG2.len();

    let sender = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let mut receiver = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    receiver
        .connect(sender.local_addr().unwrap())
        .await
        .unwrap();

    let sent = &sender
        .try_send_to(MSG, receiver.local_addr().unwrap())
        .unwrap();
    assert_eq!(sent, &MSG_LEN);
    let mut buf = [0u8; 32];
    let mut received = receiver.recv(&mut buf[..]).await.unwrap();

    sender
        .connect(receiver.local_addr().unwrap())
        .await
        .unwrap();
    let sent = &sender.try_send(MSG2).unwrap();
    assert_eq!(sent, &MSG2_LEN);
    received += receiver.recv(&mut buf[..]).await.unwrap();

    std::thread::spawn(move || {
        let sent = &sender.try_send(MSG).unwrap();
        assert_eq!(sent, &MSG_LEN);
    })
    .join()
    .unwrap();
    received += receiver.recv(&mut buf[..]).await.unwrap();

    assert_eq!(received, MSG_LEN * 2 + MSG2_LEN);
}

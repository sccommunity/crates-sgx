//! Tests for the never channel flavor.

use std::thread;
use std::time::{Duration, Instant};
use crates_unittest::test_case;
use std::prelude::v1::*;
use crossbeam_channel::{never, select, tick, unbounded};

fn ms(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

#[test_case]
fn smoke() {
    select! {
        recv(never::<i32>()) -> _ => panic!(),
        default => {}
    }
}

#[test_case]
fn optional() {
    let (s, r) = unbounded::<i32>();
    s.send(1).unwrap();
    s.send(2).unwrap();

    let mut r = Some(&r);
    select! {
        recv(r.unwrap_or(&never())) -> _ => {}
        default => panic!(),
    }

    r = None;
    select! {
        recv(r.unwrap_or(&never())) -> _ => panic!(),
        default => {}
    }
}

#[test_case]
fn tick_n() {
    let mut r = tick(ms(100));
    let mut step = 0;

    loop {
        select! {
            recv(r) -> _ => step += 1,
            default(ms(500)) => break,
        }

        if step == 10 {
            r = never();
        }
    }

    assert_eq!(step, 10);
}

#[test_case]
fn capacity() {
    let r = never::<i32>();
    assert_eq!(r.capacity(), Some(0));
}

#[test_case]
fn len_empty_full() {
    let r = never::<i32>();
    assert_eq!(r.len(), 0);
    assert_eq!(r.is_empty(), true);
    assert_eq!(r.is_full(), true);
}

#[test_case]
fn try_recv() {
    let r = never::<i32>();
    assert!(r.try_recv().is_err());

    thread::sleep(ms(100));
    assert!(r.try_recv().is_err());
}

#[test_case]
fn recv_timeout() {
    let start = Instant::now();
    let r = never::<i32>();

    assert!(r.recv_timeout(ms(100)).is_err());
    let now = Instant::now();
    assert!(now - start >= ms(100));
    assert!(now - start <= ms(150));

    assert!(r.recv_timeout(ms(100)).is_err());
    let now = Instant::now();
    assert!(now - start >= ms(200));
    assert!(now - start <= ms(250));
}

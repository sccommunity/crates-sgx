#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]
#![cfg(unix)]

use crate::tests::support::signal::send_signal;
use tokio::runtime::Runtime;
use tokio::signal::unix::{signal, SignalKind};

use std::sync::mpsc::channel;
use std::thread;
use std::string::ToString;
use crates_unittest::test_case;
use sgx_libc as libc;
use std::prelude::v1::*;

#[test_case]
fn multi_loop() {
    // An "ordinary" (non-future) channel
    let (sender, receiver) = channel();
    // Run multiple times, to make sure there are no race conditions
    for _ in 0..10 {
        // Run multiple event loops, each one in its own thread
        let threads: Vec<_> = (0..4)
            .map(|_| {
                let sender = sender.clone();
                thread::spawn(move || {
                    let mut rt = rt();
                    let _ = rt.block_on(async {
                        let mut signal = signal(SignalKind::hangup()).unwrap();
                        sender.send(()).unwrap();
                        signal.recv().await
                    });
                })
            })
            .collect();
        // Wait for them to declare they're ready
        for &_ in threads.iter() {
            receiver.recv().unwrap();
        }
        // Send a signal
        send_signal(libc::SIGHUP);
        // Make sure the threads terminated correctly
        for t in threads {
            t.join().unwrap();
        }
    }
}

fn rt() -> Runtime {
    tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap()
}

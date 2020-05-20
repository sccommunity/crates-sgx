<<<<<<< HEAD:tokio/src/sync/tests/loom_list.rs
use crate::sync::mpsc::list;
=======
extern crate futures;
#[macro_use]
extern crate loom;

macro_rules! if_fuzz {
    ($($t:tt)*) => {
        $($t)*
    }
}

#[path = "../src/mpsc/list.rs"]
#[allow(warnings)]
mod list;

#[path = "../src/mpsc/block.rs"]
#[allow(warnings)]
mod block;

const BLOCK_CAP: usize = 2;
>>>>>>> e096857bc25c075b1b62e88811fc6fc0c42905b5:tokio-sync/tests/fuzz_list.rs

use loom::thread;
use std::sync::Arc;

#[test]
fn smoke() {
    use crate::sync::mpsc::block::Read::*;

    const NUM_TX: usize = 2;
    const NUM_MSG: usize = 2;

    loom::model(|| {
        let (tx, mut rx) = list::channel();
        let tx = Arc::new(tx);

        for th in 0..NUM_TX {
            let tx = tx.clone();

            thread::spawn(move || {
                for i in 0..NUM_MSG {
                    tx.push((th, i));
                }
            });
        }

        let mut next = vec![0; NUM_TX];

        loop {
            match rx.pop(&tx) {
                Some(Value((th, v))) => {
                    assert_eq!(v, next[th]);
                    next[th] += 1;

                    if next.iter().all(|&i| i == NUM_MSG) {
                        break;
                    }
                }
                Some(Closed) => {
                    panic!();
                }
                None => {
                    thread::yield_now();
                }
            }
        }
    });
}

extern crate futures;
extern crate tokio_executor;
extern crate tokio_timer;


use super::support::*;
use futures::{prelude::*, sync::mpsc};
use tokio_timer::throttle::Throttle;
use crates_unittest::test_case;
use std::prelude::v1::*;


#[test_case]
fn throttle() {
    mocked(|timer, _| {
        let (tx, rx) = mpsc::unbounded();
        let mut stream = Throttle::new(rx, ms(1));

        assert_not_ready!(stream);

        for i in 0..3 {
            tx.unbounded_send(i).unwrap();
        }
        for i in 0..3 {
            assert_ready_eq!(stream, Some(i));
            assert_not_ready!(stream);

            advance(timer, ms(1));
        }

        assert_not_ready!(stream);
    });
}

#[test_case]
fn throttle_dur_0() {
    mocked(|_, _| {
        let (tx, rx) = mpsc::unbounded();
        let mut stream = Throttle::new(rx, ms(0));

        assert_not_ready!(stream);

        for i in 0..3 {
            tx.unbounded_send(i).unwrap();
        }
        for i in 0..3 {
            assert_ready_eq!(stream, Some(i));
        }

        assert_not_ready!(stream);
    });
}

extern crate futures;
extern crate tokio_executor;
extern crate tokio_timer;


use super::support::*;

use tokio_timer::*;

use futures::sync::{mpsc, oneshot};
use futures::{future, Future, Stream};
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn simultaneous_deadline_future_completion() {
    mocked(|_, time| {
        // Create a future that is immediately ready
        let fut = future::ok::<_, ()>(());

        // Wrap it with a deadline
        let mut fut = Timeout::new_at(fut, time.now());

        // Ready!
        assert_ready!(fut);
    });
}

#[test_case]
fn completed_future_past_deadline() {
    mocked(|_, time| {
        // Create a future that is immediately ready
        let fut = future::ok::<_, ()>(());

        // Wrap it with a deadline
        let mut fut = Timeout::new_at(fut, time.now() - ms(1000));

        // Ready!
        assert_ready!(fut);
    });
}

#[test_case]
fn future_and_deadline_in_future() {
    mocked(|timer, time| {
        // Not yet complete
        let (tx, rx) = oneshot::channel();

        // Wrap it with a deadline
        let mut fut = Timeout::new_at(rx, time.now() + ms(100));

        // Ready!
        assert_not_ready!(fut);

        // Turn the timer, it runs for the elapsed time
        advance(timer, ms(90));

        assert_not_ready!(fut);

        // Complete the future
        tx.send(()).unwrap();

        assert_ready!(fut);
    });
}

#[test_case]
fn future_and_timeout_in_future() {
    mocked(|timer, _time| {
        // Not yet complete
        let (tx, rx) = oneshot::channel();

        // Wrap it with a deadline
        let mut fut = Timeout::new(rx, ms(100));

        // Ready!
        assert_not_ready!(fut);

        // Turn the timer, it runs for the elapsed time
        advance(timer, ms(90));

        assert_not_ready!(fut);

        // Complete the future
        tx.send(()).unwrap();

        assert_ready!(fut);
    });
}

#[test_case]
fn deadline_now_elapses() {
    mocked(|_, time| {
        let fut = future::empty::<(), ()>();

        // Wrap it with a deadline
        let mut fut = Timeout::new_at(fut, time.now());

        assert_elapsed!(fut);
    });
}

#[test_case]
fn deadline_future_elapses() {
    mocked(|timer, time| {
        let fut = future::empty::<(), ()>();

        // Wrap it with a deadline
        let mut fut = Timeout::new_at(fut, time.now() + ms(300));

        assert_not_ready!(fut);

        advance(timer, ms(300));

        assert_elapsed!(fut);
    });
}

#[test_case]
fn future_errors_first() {
    mocked(|_, time| {
        let fut = future::err::<(), ()>(());

        // Wrap it with a deadline
        let mut fut = Timeout::new_at(fut, time.now() + ms(100));

        // Ready!
        assert!(fut.poll().unwrap_err().is_inner());
    });
}

#[test_case]
fn stream_and_timeout_in_future() {
    mocked(|timer, _time| {
        // Not yet complete
        let (tx, rx) = mpsc::unbounded();

        // Wrap it with a deadline
        let mut stream = Timeout::new(rx, ms(100));

        // Not ready
        assert_not_ready!(stream);

        // Turn the timer, it runs for the elapsed time
        advance(timer, ms(90));

        assert_not_ready!(stream);

        // Complete the future
        tx.unbounded_send(()).unwrap();

        let item = assert_ready!(stream);
        assert!(item.is_some());
    });
}

#[test_case]
fn idle_stream_timesout_periodically() {
    mocked(|timer, _time| {
        // Not yet complete
        let (_tx, rx) = mpsc::unbounded::<()>();

        // Wrap it with a deadline
        let mut stream = Timeout::new(rx, ms(100));

        // Not ready
        assert_not_ready!(stream);

        // Turn the timer, it runs for the elapsed time
        advance(timer, ms(100));

        assert_elapsed!(stream);
        // Stream's timeout should reset
        assert_not_ready!(stream);

        // Turn the timer, it runs for the elapsed time
        advance(timer, ms(100));
        assert_elapsed!(stream);
    });
}

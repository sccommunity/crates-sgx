

use mio::net::TcpStream;
use mio::{event, Token, Waker};
use std::time::Duration;


use super::util::init_with_poll;
use crates_unittest::test_case;
use std::prelude::v1::*;
const WAKE_TOKEN: Token = Token(10);

#[test_case]
fn assert_event_source_implemented_for() {
    fn assert_event_source<E: event::Source>() {}

    assert_event_source::<Box<dyn event::Source>>();
    assert_event_source::<Box<TcpStream>>();
}

#[test_case]
fn events_all() {
    let (mut poll, mut events) = init_with_poll();
    assert_eq!(events.capacity(), 16);
    assert!(events.is_empty());

    let waker = Waker::new(poll.registry(), WAKE_TOKEN).unwrap();

    waker.wake().expect("unable to wake");
    poll.poll(&mut events, Some(Duration::from_millis(100)))
        .unwrap();

    assert!(!events.is_empty());

    for event in events.iter() {
        assert_eq!(event.token(), WAKE_TOKEN);
        assert!(event.is_readable());
    }

    events.clear();
    assert!(events.is_empty());
}

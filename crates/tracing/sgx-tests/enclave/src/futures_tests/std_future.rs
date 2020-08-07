


use tracing::{subscriber::with_default, Level};
use tracing_futures::Instrument;
use std::prelude::v1::*;
use crates_unittest::test_case;


use crate::futures_tests::supports::*;
use crate::futures_tests::support::*;
#[test_case]
fn enter_exit_is_reasonable() {
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
        .run_with_handle();
    with_default(subscriber, || {
        let future = PollN::new_ok(2).instrument(tracing::span!(Level::TRACE, "foo"));
        block_on_future(future).unwrap();
    });
    handle.assert_finished();
}

#[test_case]
fn error_ends_span() {
    let (subscriber, handle) = subscriber::mock()
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .enter(span::mock().named("foo"))
        .exit(span::mock().named("foo"))
        .drop_span(span::mock().named("foo"))
        .done()
        .run_with_handle();
    with_default(subscriber, || {
        let future = PollN::new_err(2).instrument(tracing::span!(Level::TRACE, "foo"));
        block_on_future(future).unwrap_err();
    });
    handle.assert_finished();
}

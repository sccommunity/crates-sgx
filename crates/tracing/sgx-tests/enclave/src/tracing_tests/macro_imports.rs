use tracing::Level;
use std::prelude::v1::*;
use crates_unittest::test_case;
#[test_case]
fn prefixed_span_macros() {
    tracing::span!(Level::DEBUG, "foo");
    tracing::trace_span!("foo");
    tracing::debug_span!("foo");
    tracing::info_span!("foo");
    tracing::warn_span!("foo");
    tracing::error_span!("foo");
}

#[test_case]
fn prefixed_event_macros() {
    tracing::event!(Level::DEBUG, "foo");
    tracing::trace!("foo");
    tracing::debug!("foo");
    tracing::info!("foo");
    tracing::warn!("foo");
    tracing::error!("foo");
}

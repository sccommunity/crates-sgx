use tracing_log::{log::LevelFilter, LogTracer};
use std::prelude::v1::*;
use crates_unittest::test_case;
/// This test makes sure we can access `log::LevelFilter` through the `tracing_log` crate and don't
/// have to depend on `log` separately.
///
/// See https://github.com/tokio-rs/tracing/issues/552.
#[test_case]
fn can_initialize_log_tracer_with_level() {
    LogTracer::init_with_filter(LevelFilter::Error).unwrap();
}

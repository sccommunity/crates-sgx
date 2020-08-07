use std::prelude::v1::*;
use crates_unittest::test_case;
use crate::subscriber_tests::support::*;
use tracing_subscriber::prelude::*;

#[test_case]
fn init_ext_works() {
    let (subscriber, finished) = subscriber::mock()
        .event(
            event::mock()
                .at_level(tracing::Level::INFO)
                .with_target("init_works"),
        )
        .done()
        .run_with_handle();

    let _guard = subscriber.set_default();
    tracing::info!(target: "init_works", "it worked!");
    finished.assert_finished();
}

#[test_case]
fn builders_are_init_ext() {
    tracing_subscriber::fmt().set_default();
    let _ = tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .try_init();
}

#[test_case]
fn layered_is_init_ext() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new("foo=info"))
        .set_default();
}

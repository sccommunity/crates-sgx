
use tracing_core::dispatcher::*;
use std::prelude::v1::*;
use crates_unittest::test_case;
use crate::core_tests::common::{ TestSubscriberA, TestSubscriberB };
#[test_case]
fn set_default_dispatch() {
    set_global_default(Dispatch::new(TestSubscriberA)).expect("global dispatch set failed");
    get_default(|current| {
        assert!(
            current.is::<TestSubscriberA>(),
            "global dispatch get failed"
        )
    });

    let guard = set_default(&Dispatch::new(TestSubscriberB));
    get_default(|current| assert!(current.is::<TestSubscriberB>(), "set_default get failed"));

    // Drop the guard, setting the dispatch back to the global dispatch
    drop(guard);

    get_default(|current| {
        assert!(
            current.is::<TestSubscriberA>(),
            "global dispatch get failed"
        )
    });
}


#[test_case]
fn nested_set_default() {
    let _guard = set_default(&Dispatch::new(TestSubscriberA));
    get_default(|current| {
        assert!(
            current.is::<TestSubscriberA>(),
            "set_default for outer subscriber failed"
        )
    });

    let inner_guard = set_default(&Dispatch::new(TestSubscriberB));
    get_default(|current| {
        assert!(
            current.is::<TestSubscriberB>(),
            "set_default inner subscriber failed"
        )
    });

    drop(inner_guard);
    get_default(|current| {
        assert!(
            current.is::<TestSubscriberA>(),
            "set_default outer subscriber failed"
        )
    });
}


use crate::core_tests::common::{ TestSubscriberA };

use tracing_core::dispatcher::*;
use std::prelude::v1::*;
use crates_unittest::test_case;
#[test_case]
fn global_dispatch() {
    set_global_default(Dispatch::new(TestSubscriberA)).expect("global dispatch set failed");
    get_default(|current| {
        assert!(
            current.is::<TestSubscriberA>(),
            "global dispatch get failed"
        )
    });

    #[cfg(feature = "std")]
    with_default(&Dispatch::new(TestSubscriberB), || {
        get_default(|current| {
            assert!(
                current.is::<TestSubscriberB>(),
                "thread-local override of global dispatch failed"
            )
        });
    });

    get_default(|current| {
        assert!(
            current.is::<TestSubscriberA>(),
            "reset to global override failed"
        )
    });

    set_global_default(Dispatch::new(TestSubscriberA))
        .expect_err("double global dispatch set succeeded");
}

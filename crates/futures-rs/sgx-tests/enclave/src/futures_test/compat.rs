#![cfg(feature = "compat")]
use std::string::ToString;
use crates_unittest::test_case;
use tokio::timer::Delay;
use tokio::runtime::Runtime;
use std::time::Instant;
use futures::prelude::*;
use futures::compat::Future01CompatExt;

#[test_case]
fn can_use_01_futures_in_a_03_future_running_on_a_01_executor() {
    let f = async {
        Delay::new(Instant::now()).compat().await
    };

    let mut runtime = Runtime::new().unwrap();
    runtime.block_on(f.boxed().compat()).unwrap();
}

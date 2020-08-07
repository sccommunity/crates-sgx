#![warn(rust_2018_idioms)]

use tokio::time::{delay_until, Duration, Instant};
use tokio_test::block_on;
use std::string::ToString;
use crates_unittest::test_case;
#[test_case]
fn async_block() {
    assert_eq!(4, block_on(async { 4 }));
}

async fn five() -> u8 {
    5
}

#[test_case]
fn async_fn() {
    assert_eq!(5, block_on(five()));
}

#[test_case]
fn test_delay() {
    let deadline = Instant::now() + Duration::from_millis(100);

    block_on(async {
        delay_until(deadline).await;
    });
}

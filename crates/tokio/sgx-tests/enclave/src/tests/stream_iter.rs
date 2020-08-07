use tokio::stream;
use tokio_test::task;

use std::iter;
use std::string::ToString;
use crates_unittest::test_case;
#[crates_unittest::test]
async fn coop() {
    let mut stream = task::spawn(stream::iter(iter::repeat(1)));

    for _ in 0..10_000 {
        if stream.poll_next().is_pending() {
            assert!(stream.is_woken());
            return;
        }
    }

    panic!("did not yield");
}

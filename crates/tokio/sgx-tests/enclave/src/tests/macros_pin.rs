use std::string::ToString;
use crates_unittest::test_case;

async fn one() {}
async fn two() {}

#[crates_unittest::test]
async fn multi_pin() {
    tokio::pin! {
        let f1 = one();
        let f2 = two();
    }

    (&mut f1).await;
    (&mut f2).await;
}

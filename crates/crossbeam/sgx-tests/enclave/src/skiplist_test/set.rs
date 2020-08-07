use crossbeam_skiplist::SkipSet;
use crates_unittest::test_case;
use std::prelude::v1::*;
#[test_case]
fn smoke() {
    let m = SkipSet::new();
    m.insert(1);
    m.insert(5);
    m.insert(7);
}

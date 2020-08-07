use ring::{signature, test};
use std::prelude::v1::*;
use crates_unittest::test_case;


#[test_case]
fn signature_impl_test() {
    test::compile_time_assert_clone::<signature::Signature>();
    test::compile_time_assert_copy::<signature::Signature>();
    test::compile_time_assert_send::<signature::Signature>();
    test::compile_time_assert_sync::<signature::Signature>();
}

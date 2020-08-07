use std::prelude::v1::*;
use crates_unittest::test_case;
#[test_case]
fn error_impl_std_error_error_test() {
    use ring::{error, test};
    test::compile_time_assert_std_error_error::<error::Unspecified>();
    test::compile_time_assert_std_error_error::<error::KeyRejected>();
}

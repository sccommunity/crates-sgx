#![cfg(feature = "executor")] // executor::
#![cfg(feature = "async-await")] // try_join!
#![deny(unreachable_code)]
use std::string::ToString;
use crates_unittest::test_case;
use std::prelude::v1::*;
use futures::{try_join, executor::block_on};

// TODO: This abuses https://github.com/rust-lang/rust/issues/58733 in order to
// test behaviour of the `try_join!` macro with the never type before it is
// stabilized. Once `!` is again stabilized this can be removed and replaced
// with direct use of `!` below where `Never` is used.
trait MyTrait {
    type Output;
}
impl<T> MyTrait for fn() -> T {
    type Output = T;
}
type Never = <fn() -> ! as MyTrait>::Output;


#[test_case]
fn try_join_never_error() {
    block_on(async {
        let future1 = async { Ok::<(), Never>(()) };
        let future2 = async { Ok::<(), Never>(()) };
        try_join!(future1, future2)
    })
    .unwrap();
}

#[test_case]
fn try_join_never_ok() {
    block_on(async {
        let future1 = async { Err::<Never, ()>(()) };
        let future2 = async { Err::<Never, ()>(()) };
        try_join!(future1, future2)
    })
    .unwrap_err();
}
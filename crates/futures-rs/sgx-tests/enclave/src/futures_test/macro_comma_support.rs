use std::string::ToString;
use crates_unittest::test_case;
#[cfg(feature = "executor")]
#[test_case]
fn ready() {
    use futures::{
        executor::block_on,
        future,
        task::Poll,
        ready,
    };

    block_on(future::poll_fn(|_| {
        ready!(Poll::Ready(()),);
        Poll::Ready(())
    }))
}

#[cfg(all(feature = "executor", feature = "async-await"))]
#[test_case]
fn poll() {
    use futures::{
        executor::block_on,
        future::FutureExt,
        poll,
    };

    block_on(async {
        let _ = poll!(async {}.boxed(),);
    })
}

#[cfg(all(feature = "executor", feature = "async-await"))]
#[test_case]
fn join() {
    use futures::{
        executor::block_on,
        join
    };

    block_on(async {
        let future1 = async { 1 };
        let future2 = async { 2 };
        join!(future1, future2,);
    })
}

#[cfg(all(feature = "executor", feature = "async-await"))]
#[test_case]
fn try_join() {
    use futures::{
        executor::block_on,
        future::FutureExt,
        try_join,
    };

    block_on(async {
        let future1 = async { 1 }.never_error();
        let future2 = async { 2 }.never_error();
        try_join!(future1, future2,)
    })
    .unwrap();
}

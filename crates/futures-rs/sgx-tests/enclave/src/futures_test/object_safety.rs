use std::string::ToString;
use crates_unittest::test_case;
fn assert_is_object_safe<T>() {}

#[test_case]
fn future() {
    // `FutureExt`, `TryFutureExt` and `UnsafeFutureObj` are not object safe.
    use futures::future::{FusedFuture, Future, TryFuture};

    assert_is_object_safe::<&dyn Future<Output = ()>>();
    assert_is_object_safe::<&dyn FusedFuture<Output = ()>>();
    assert_is_object_safe::<&dyn TryFuture<Ok = (), Error = (), Output = Result<(), ()>>>();
}

#[test_case]
fn stream() {
    // `StreamExt` and `TryStreamExt` are not object safe.
    use futures::stream::{FusedStream, Stream, TryStream};

    assert_is_object_safe::<&dyn Stream<Item = ()>>();
    assert_is_object_safe::<&dyn FusedStream<Item = ()>>();
    assert_is_object_safe::<&dyn TryStream<Ok = (), Error = (), Item = Result<(), ()>>>();
}

#[test_case]
fn sink() {
    // `SinkExt` is not object safe.
    use futures::sink::Sink;

    assert_is_object_safe::<&dyn Sink<(), Error = ()>>();
}

#[cfg(feature = "std")] // futures::io
#[test_case]
fn io() {
    // `AsyncReadExt`, `AsyncWriteExt`, `AsyncSeekExt` and `AsyncBufReadExt` are not object safe.
    use futures::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

    assert_is_object_safe::<&dyn AsyncRead>();
    assert_is_object_safe::<&dyn AsyncWrite>();
    assert_is_object_safe::<&dyn AsyncSeek>();
    assert_is_object_safe::<&dyn AsyncBufRead>();
}

#[test_case]
fn task() {
    // `ArcWake`, `SpawnExt` and `LocalSpawnExt` are not object safe.
    use futures::task::{LocalSpawn, Spawn};

    assert_is_object_safe::<&dyn Spawn>();
    assert_is_object_safe::<&dyn LocalSpawn>();
}

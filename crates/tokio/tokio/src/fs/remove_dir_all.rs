use crate::fs::asyncify;

use std::io;
use std::path::Path;
use std::prelude::v1::*;

/// Removes a directory at this path, after removing all its contents. Use carefully!
///
/// This is an async version of [`std::fs::remove_dir_all`][std]
///
/// [std]: fn@std::fs::remove_dir_all
pub async fn remove_dir_all(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref().to_owned();
    asyncify(move || std::fs::remove_dir_all(path)).await
}

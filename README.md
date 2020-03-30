# crates-sgx

crates-sgx is a monorepo of all Rust crates ported for Teaclave SGX SDK
targeting the Intel SGX platform.

## Usage

The
[crates-sgx-example](https://github.com/universal-secure-computing-community/crates-sgx-example)
project illustrates usages of these crates for developing sgx enclave.

Here is an example of `Cargo.toml`.

```
[package]
name = "crates-sgx-example"
version = "0.1.0"
authors = ["The Universal Secure Computing Community Authors"]
edition = "2018"

[lib]
name = "crates_sgx_example"
crate-type = ["staticlib", "rlib"]

[features]
default = ["mesalock_sgx"]
mesalock_sgx = [
  "sgx_tstd",
]

[dependencies]
cfg-if = { git = "https://github.com/universal-secure-computing-community/crates-sgx.git", tag = "v0.1.0+sgx1.1.0" }
hex = { git = "https://github.com/universal-secure-computing-community/crates-sgx.git", tag = "v0.1.0+sgx1.1.0" }

sgx_tstd = { git = "https://github.com/apache/incubator-teaclave-sgx-sdk.git", tag = "v1.1.0", optional = true }
```

## Branch

- `sgx1.1.0` (default): Teaclave SGX SDK 1.1.0 (Intel SGX SDK 2.7.1)

## Version

Given a version number `x.y.z`, increment:
- `z` when adding, updating, deprecating and deleting crates of crates-sgx
- `y` when updating Teaclave SGX SDK
- `x` when making incompatible structural/API changes of crates-sgx

For example:
- crates-sgx 0.1.0: Teaclave SGX SDK 1.1.0 (Intel SGX SDK 2.7.1) + 10 SGX crates
- crates-sgx 0.1.1: Teaclave SGX SDK 1.1.0 (Intel SGX SDK 2.7.1) + 20 SGX crates
- crates-sgx 0.2.0: Teaclave SGX SDK 1.2.0 (Intel SGX SDK 2.8) + 20 SGX crates

## Tag

Tags follow this convention `v{x.y.z}+sgx{x.y.z}`, where:
- `v{x.y.z}` is the version of crates-sgx
- `sgx{x.y.z}` is the version of Teaclave SGX SDK

## Contributing

- Propose a crate you paln to port in issue and then port crates and
  corresponding tests for Teaclave SGX SDK.
- Create a pull request stating crate name, version, license and upstream
  commit hash (usually a public release) which your port based on.
- Provide a diff of your changes for reviewing and auditing.
- Add metadata in the crates section of `README.md` and update `CHANGELOG.md`.

## Porting

1. Fix dependencies: add SGX related dependencies in the Cargo.toml. For example:
```
[dependencies]
sgx_tstd = { git = "https://github.com/apache/incubator-teaclave-sgx-sdk.git", tag = "v1.1.0", optional = true }

// Replace hex = "0.4.2", and pay attention to the tag
hex = { git = "https://github.com/universal-secure-computing-community/crates-sgx.git", tag = "0.1.0" }
```

2. Fix features: add a feature to enable SGX's standard library. For example:

```
[features]
default = ["mesalock_sgx"]
mesalock_sgx = ["sgx_tstd"]
```

3. Add headers in the `lib.rs` to enable the std. For example:

```
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;
```

4. Fix standard library issues. For example:

```
// use std::sync::{Arc, Mutex};
use std::sync::{Arc, SgxMutex as Mutex};
```
5. Tests including unit tests and functional tests should be also ported for SGX
   std.
   [Here](https://github.com/universal-secure-computing-community/crates-sgx/commit/01e0595f66af87a0c3631360696217dbbae90f14)
   is an example to port tests. Basically, you can use `utils/sgx-tests`
   template as a test driver for testing in SGX enclave.

6. There are other cases like file system APIs, and untrusted calls you need to
   review and handle properly. Feel free to submit an issue to discussion.

## Crates

- `{crate name}`, `{version}`, `{license}`, `{upstream commit hash base}`
- cfg-if, 0.1.10, MIT/Apache-2.0, 4484a6faf816ff8058088ad857b0c6bb2f4b02b2
- itoa, 0.4.5, MIT/Apache-2.0, 0ecba421fa2f278431d1e0ab607d9311ad2c1541
- rust-hex, 0.4.2, MIT/Apache-2.0, be0c32f9c8938ca0359bbb0d1477e31b07cb3358

## License

crates-sgx is provided under the Apache License 2.0. We only accept crates
provided under ["Apache-like"](https://www.apache.org/legal/resolved.html)
license.

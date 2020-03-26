# crates-sgx

crates-sgx is a monorepo of all Rust crates ported for Teaclave SGX SDK
targeting the Intel SGX platform.

## Branch

- `sgx1.1.0` (default): Teaclave SGX SDK 1.1.0 (Intel SGX SDK 2.7.1)

## Version

Given a version number X.Y.Z, increment:
- X when making incompatible structural/API changes
- Y when updating Teaclave SGX SDK
- Z when adding, updating, deprecating and deleting crates

For example:
- crates-sgx 0.1.0: Teaclave SGX SDK 1.1.0 (Intel SGX SDK 2.7.1) + 10 SGX crates
- crates-sgx 0.1.1: Teaclave SGX SDK 1.1.0 (Intel SGX SDK 2.7.1) + 20 SGX crates
- crates-sgx 0.2.0: Teaclave SGX SDK 1.2.0 (Intel SGX SDK 2.8) + 20 SGX crates

## Tag

Tags follow this convention `v{x.y.z}+sgx{x.y.z}`, where:
- `v{x.y.z}` is the version of crates-sgx
- `sgx{x.y.z}` is the version of Teaclave SGX SDK

## Crates

- crate name, version, license, upstream commit hash base
- cfg-if, 0.1.10, MIT/Apache-2.0, 4484a6faf816ff8058088ad857b0c6bb2f4b02b2
- rust-hex, 0.4.2, MIT/Apache-2.0, be0c32f9c8938ca0359bbb0d1477e31b07cb3358

## License

crates-sgx is provided under the Apache License 2.0. We only accept crates
provided under ["Apache-like"](https://www.apache.org/legal/resolved.html)
license.

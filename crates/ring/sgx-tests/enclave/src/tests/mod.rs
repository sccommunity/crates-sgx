



macro_rules! test_file {
    ($file_name:expr) => {
        ring::test::File {
            file_name: $file_name,
            contents: include_str!($file_name),
        }
    };
}


pub struct File<'a> {
    /// The name (path) of the file.
    pub file_name: &'a str,

    /// The contents of the file.
    pub contents: &'a str,
}

mod aead_tests;
mod agreement_tests;
mod constant_time_tests;
mod digest_tests;
mod ecdsa_tests;
mod ed25519_tests;
mod error_tests;
mod hkdf_tests;
mod hmac_tests;
mod pbkdf2_tests;
mod quic_tests;
mod rand_tests;
mod rsa_tests;
mod signature_tests;

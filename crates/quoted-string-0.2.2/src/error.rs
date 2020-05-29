use std::result::{Result as StdResult};

quick_error! {

    /// Error returned by the quoting algorithm
    #[derive(Debug)]
    pub enum Error {
        /// Input contained a character which could not be quoted
        UnquotableCharacter(ch: char) {
            description("tried to quote a character which can not be quoted")
            display("the character {:?} can not be quoted", ch)
        }

        /// Input contained non us-ascii character, but was required to be us-ascii only
        NonUsAsciiInput {
            description("text is expected to be us-ascii only but wasn't")
        }
    }
}

/// Result type alias defaulting the error to `quoted_string::error::Error`
pub type Result<T> = StdResult<T, Error>;
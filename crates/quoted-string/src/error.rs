//! module containing all errors
use std::error::{Error as StdError};
use std::fmt::{self, Display};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CoreError {
    AdvancedFailedAutomaton,
    QuotedStringAlreadyEnded,
    UnquoteableCharQuoted,
    DoesNotStartWithDQuotes,
    DoesNotEndWithDQuotes,
    InvalidChar,
    ZeroSizedValue
}

impl CoreError {

    pub fn id(&self) -> u8 {
        use self::CoreError::*;
        match *self {
            AdvancedFailedAutomaton => 0,
            QuotedStringAlreadyEnded => 1,
            UnquoteableCharQuoted => 2,
            DoesNotStartWithDQuotes => 3,
            DoesNotEndWithDQuotes => 4,
            InvalidChar => 5,
            ZeroSizedValue => 6,
        }
    }

    pub fn from_id(id: u8) -> Option<Self> {
        use self::CoreError::*;
        Some(match id {
            0 => AdvancedFailedAutomaton,
            1 => QuotedStringAlreadyEnded,
            2 => UnquoteableCharQuoted,
            3 => DoesNotStartWithDQuotes,
            4 => DoesNotEndWithDQuotes,
            5 => InvalidChar,
            6 => ZeroSizedValue,
            _ => return None
        })
    }
}
impl Display for CoreError {
    fn fmt(&self, fter: &mut fmt::Formatter) -> fmt::Result {
        fter.write_str(self.description())
    }
}

impl StdError for CoreError {
    fn description(&self) -> &'static str {
        use self::CoreError::*;
        match *self {
            AdvancedFailedAutomaton =>
                "advanced automaton after it entered the failed state",
            QuotedStringAlreadyEnded =>
                "continued to use the automaton after the end of the quoted string was found",
            UnquoteableCharQuoted =>
                "a char was escaped with a quoted-pair which can not be represented with a quoted-pair",
            DoesNotStartWithDQuotes =>
                "quoted string did not start with \"",
            DoesNotEndWithDQuotes =>
                "quoted string did not end with \"",
            InvalidChar =>
                "char can not be represented in a quoted string (without encoding)",
            ZeroSizedValue =>
                "value had a size of zero chars/bytes but has to have at last one"
        }
    }
}

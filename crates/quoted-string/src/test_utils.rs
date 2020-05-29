//! provides an example implementation of quoted string spec's
use std::default::Default;

use spec::{
    GeneralQSSpec,
    QuotingClassifier, QuotingClass,
    ParsingImpl,
    State,
    PartialCodePoint,
    WithoutQuotingValidator
};
use error::CoreError;

#[derive(Copy, Clone, Debug)]
pub struct TestSpec;

impl GeneralQSSpec for TestSpec {
    type Quoting = Self;
    type Parsing = TestParsingImpl;
}

impl QuotingClassifier for TestSpec {
    fn classify_for_quoting(pcp: PartialCodePoint) -> QuotingClass {
        if !is_valid_pcp(pcp) {
            QuotingClass::Invalid
        } else {
            match pcp.as_u8() {
                b'"' | b'\\' => QuotingClass::NeedsQuoting,
                _ => QuotingClass::QText
            }
        }
    }
}

fn is_valid_pcp(pcp: PartialCodePoint) -> bool {
    let bch = pcp.as_u8();
    b' ' <= bch && bch <= b'~'
}

/// a parsing implementations which allows non semantic stange thinks in it for testing purpose
///
/// basically you can have a non-semantic section starting with `←` ending with `→` which has
/// a number of `+` followed by the same number of `-`.
///
/// E.g. `"some think \n+++---\n"`
///
/// This naturally makes no sense, but is a simple way to test if the custom state is used
/// correctly, there are some quoted string impl which need custom state as they can e.g.
/// have non semantic soft line brakes (which are slight more complex to implement and less
/// visible in error messages, so I used this think here)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum TestParsingImpl {
    StrangeInc(usize),
    StrangeDec(usize)
}

impl ParsingImpl for TestParsingImpl {
    fn can_be_quoted(bch: PartialCodePoint) -> bool {
        is_valid_pcp(bch)
    }
    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        if bch.as_u8() == b'\n' {
            Ok((State::Custom(TestParsingImpl::StrangeInc(0)), false))
        } else if is_valid_pcp(bch) {
            Ok((State::Normal, true))
        } else {
            Err(CoreError::InvalidChar)
        }
    }

    fn advance(&self, pcp: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        use self::TestParsingImpl::*;
        let bch = pcp.as_u8();
        match *self {
            StrangeInc(v) => {
                if bch == b'\n' {
                    Ok((State::Normal, false))
                } else if bch == b'+' {
                    Ok((State::Custom(StrangeInc(v+1)), false))
                } else if bch == b'-' && v > 0{
                    Ok((State::Custom(StrangeDec(v-1)), false))
                } else {
                    Err(CoreError::InvalidChar)
                }
            },
            StrangeDec(v) => {
                if bch == b'-' && v > 0 {
                    Ok((State::Custom(StrangeDec(v-1)), false))
                } else if bch == b'\n' && v == 0 {
                    Ok((State::Normal, false))
                } else {
                    Err(CoreError::InvalidChar)
                }
            }
        }
    }
}


pub struct TestUnquotedValidator {
    pub count: usize,
    pub last_was_dot: bool
}
impl Default for TestUnquotedValidator {
    fn default() -> Self {
        TestUnquotedValidator {
            count: 0,
            last_was_dot: true
        }
    }
}
impl TestUnquotedValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl WithoutQuotingValidator for TestUnquotedValidator {
    fn next(&mut self, pcp: PartialCodePoint) -> bool {
        let bch = pcp.as_u8();
        let lwd = self.last_was_dot;
        let res = match bch {
            b'a'...b'z' => {
                self.last_was_dot = false;
                true
            }
            b'.' if !lwd => {
                self.last_was_dot = true;
                true
            }
            _ => false
        };
        if res {
            self.count += 1;
        }
        res
    }
    fn end(&self) -> bool {
        self.count == 6 && !self.last_was_dot
    }
}
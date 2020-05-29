//! This module contains types for specifying what kind of quoted string is used
use std::fmt::Debug;
use error::CoreError;

/// type to specify the quoting classifier and parsing implementation
///
/// This is normally a zero-sized type.
pub trait GeneralQSSpec: Clone+Debug {
    type Quoting: QuotingClassifier;
    type Parsing: ParsingImpl;
}

/// Type to provide a quoting classification method.
///
/// This is normally a zero-sized type.
pub trait QuotingClassifier {

    /// Returns the "class" the partial code point belongs too
    ///
    /// This is either `QTest`, `NeedsQuoting` or `Invalid`.
    /// As this function accepts `PartialCodePoint` it can not
    /// differ between different utf-8 characters, which happens
    /// to be fine for all supported quoting use cases.
    fn classify_for_quoting(pcp: PartialCodePoint) -> QuotingClass;
}

/// Represents if a char can be contained in a quoted string and if it needs escapeing
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum QuotingClass {
    /// The char can be represented in a quoted string
    QText,
    /// The char can be represented but needs quoting (e.g. the " in `r#"bla\"blo"#`)
    NeedsQuoting,
    /// The char is invalid (e.g. a CTL char)
    Invalid
}

/// Used to validate if a string is valid without beeing quoted.
///
/// Depending on the complexity of the underlying grammar this types
/// implementing this trait might have an internal state, through
/// they have to be careful wrt. the semantics of `next` when mutating
/// it.
///
/// Types impl this trait are normally not expected to be reused between
/// multiple calls to `quoted_if_needed` (or whoever uses it). And might
/// keep track of some meta-information as they are passed normally to the
/// consuming function as `&mut`.
pub trait WithoutQuotingValidator {
    /// if next returns false, it's (self) state should NOT be modified
    /// i.e. calling .end() after next(..) and returning false corresponds
    /// to the input sequence _until_ next(..) was false, not including
    /// the `pcp` from the last `next` call
    fn next(&mut self, pcp: PartialCodePoint) -> bool;

    /// this is called once the validation through next ended
    ///
    /// - the validation might end because there is no more input
    /// - but it also might end because next returned false, due to the
    ///   definition of next to not change the state if it returns false
    ///   this can and is done
    /// - it _does not_ need to validate that the length is at last 1, this
    ///   is done by the algorithm using it
    /// - so for many cases this is just true (the default impl)
    fn end(&self) -> bool { true }
}

/// State used when parsing a quoted string
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum State<T: Copy+Eq+Debug> {
    /// The initial state
    Start,
    /// The normal state
    Normal,
    /// Failed as it e.g. hit an invalid char
    Failed,
    /// start of a quoted-pair e.g. the \ of \"
    QPStart,
    /// a custom state needed for more complex quoted strings
    Custom(T),
    /// the end of the quoted string was found
    /// (this is not necessary the end of the input)
    End
}

/// This normally zero sized type provides functions for parsing a quoted string
pub trait ParsingImpl: Copy+Eq+Debug {
    fn can_be_quoted(bch: PartialCodePoint) -> bool;
    fn handle_normal_state(bch: PartialCodePoint) -> Result<(State<Self>, bool), CoreError>;
    fn advance(&self, _pcp: PartialCodePoint) -> Result<(State<Self>, bool), CoreError> {
        unreachable!("[BUG] custom state is not used, so advance is unreachable")
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ScanAutomaton<T: ParsingImpl> {
    state: State<T>,
    last_was_emit: bool
}

impl<Impl> ScanAutomaton<Impl>
    where Impl: ParsingImpl
{

    pub fn new() -> Self {
        ScanAutomaton { state: State::Start, last_was_emit: false }
    }

    pub fn did_end(&self) -> bool {
        self.state == State::End
    }

    pub fn end(&mut self) -> Result<(), CoreError> {
        if self.did_end() {
            Ok(())
        } else {
            Err(CoreError::DoesNotEndWithDQuotes.into())
        }
    }

    pub fn advance(&mut self, pcp: PartialCodePoint) -> Result<bool, CoreError> {
        match _advance_scan_automaton(self.state, pcp) {
            Ok((state, emit)) => {
                self.state = state;
                self.last_was_emit = emit;
                Ok(emit)
            },
            Err(err) => {
                self.state = State::Failed;
                Err(err)
            }
        }
    }
}

fn _advance_scan_automaton<Impl: ParsingImpl>(state: State<Impl>, pcp: PartialCodePoint)
    -> Result<(State<Impl>, bool), CoreError>
{
    use self::State::*;
    let pcp_val = pcp.as_u8();
    match state {
        Start => {
            if pcp_val == b'"' {
                Ok((Normal, false))
            } else {
                Err(CoreError::DoesNotStartWithDQuotes)
            }
        }
        Normal => {
            match pcp_val {
                b'"' => Ok((End, false)),
                b'\\' => Ok((QPStart, false)),
                _ => Impl::handle_normal_state(pcp)
            }
        }
        QPStart => {
            if Impl::can_be_quoted(pcp) {
                Ok((Normal, true))
            } else {
                Err(CoreError::UnquoteableCharQuoted.into())
            }
        }
        Custom(inner) => {
            inner.advance(pcp)
        }
        End => {
            Err(CoreError::QuotedStringAlreadyEnded.into())
        },
        Failed => Err(CoreError::AdvancedFailedAutomaton.into())
    }
}

/// A type which represents part of a utf-8 code point
///
/// It does not know which part of a code point it represents
/// (e.g. utf-8 first or later byte of a code point > 0x7f).
///
/// When used in a iteration like context it is also not guaranteed
/// to go through all utf-8 bytes, it might, or it might just represent
/// the first byte replacing all bytes > 0x7f with 0xFF.
///
/// This allows efficiently abstracting over char sequences and utf-8
/// byte sequences for tasks which are mainly focused on us-ascii and
/// treat all non ascii utf8 code points the same.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PartialCodePoint(u8);
impl PartialCodePoint {
    #[inline(always)]
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// creates a partial code point from a utf8 byte
    ///
    /// The inner value will be the byte passed in,
    /// which should not be 0xff as 0xff doesn't appear
    /// in a utf-8 byte sequence
    ///
    /// # Debug Assertions
    ///
    /// if debug assertions are enabled and 0xff is passed
    /// in this will panic as it wasn't created from a byte
    /// from a utf-8 byte sequence
    #[inline(always)]
    pub fn from_utf8_byte(u8b: u8) -> PartialCodePoint {
        debug_assert!(u8b != 0xFF, "utf8 bytes can not be 0xFF");
        PartialCodePoint(u8b)
    }

    /// creates a `PartialCodePoint` from a utf-8 code point.
    ///
    /// The inner value will be:
    ///
    /// - the char if the code point is us-ascii
    /// - 0xFF if it is larger then 0x7f i.e. non us-ascii
    #[inline]
    pub fn from_code_point(code_point: u32) -> PartialCodePoint {
        if code_point > 0x7f {
            PartialCodePoint(0xFF)
        } else {
            PartialCodePoint(code_point as u8)
        }
    }
}


/// Allows unquoted text containing only `_ | a..z | A..Z | 0..9`
#[derive(Copy, Clone, Debug)]
pub struct AsciiWordValidator;

impl WithoutQuotingValidator for AsciiWordValidator {
    fn next(&mut self, pcp: PartialCodePoint) -> bool {
        let u8val = pcp.as_u8();
        u8val.is_ascii_alphanumeric() || u8val == b'_'
    }
}
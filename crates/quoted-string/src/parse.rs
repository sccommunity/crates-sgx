use spec::{ScanAutomaton, GeneralQSSpec,  PartialCodePoint};
use error::CoreError;

/// validates if input is a valid quoted-string
///
/// in difference to parse it requires the whole input to be one quoted-string
///
/// # Example
///
/// ```
/// // use your own spec
/// use quoted_string::test_utils::TestSpec;
/// use quoted_string::validate;
///
/// assert!(validate::<TestSpec>("\"quoted string\""));
/// assert!(!validate::<TestSpec>("\"not right\"really not"));
/// ```
///
pub fn validate<Spec: GeneralQSSpec>(input: &str) -> bool {
    parse::<Spec>(input)
        .map(|res|res.tail.is_empty())
        .unwrap_or(false)
}

/// the result of successfully parsing a quoted string
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Parsed<'a> {
    /// the parsed quoted string
    pub quoted_string: &'a str,
    /// the rest of the input string, not parsed
    pub tail: &'a str
}

/// parse a quoted string starting at the begin of `input` but possible ending earlier
///
/// To check if the whole string is a quoted-string (an nothing more) you have to
/// additional check if `parsed.tail` is empty.
///
/// # Error
///
/// a error and the char index where it was triggered is returned if the input does not start
/// with a valid quoted-string.
///
/// # Example
///
/// ```
/// // use your own Spec
/// use quoted_string::test_utils::TestSpec;
/// use quoted_string::{parse, Parsed};
///
/// let parsed = parse::<TestSpec>("\"list of\"; \"quoted strings\"").unwrap();
/// assert_eq!(parsed, Parsed {
///     quoted_string: "\"list of\"",
///     tail:  "; \"quoted strings\""
/// });
/// ```
///
pub fn parse<Impl: GeneralQSSpec>(input: &str) -> Result<Parsed, (usize, CoreError)> {
    let mut automaton = ScanAutomaton::<Impl::Parsing>::new();

    for (idx, bch) in input.bytes().enumerate() {
        automaton.advance(PartialCodePoint::from_utf8_byte(bch))
            .map_err(|err| (idx, err.into()))?;

        if automaton.did_end() {
            return Ok(Parsed {
                //idx+1: idx is the idx of the ending '"' which has a byte len of 1
                quoted_string: &input[0..idx + 1],
                tail: &input[idx + 1..]
            })
        }
    }
    // if we reach heare it's a error as the closing '"' is missing.
    // So .end() will _always_ trigger an error in this position
    match automaton.end() {
        Ok(_) =>
            panic!("[BUG] automaton.did_end() == false but automaton.end() does not trigger error"),
        Err(err) => {
            Err((input.len(), err.into()))
        }
    }
}


#[cfg(test)]
mod test {

    mod parse {
        use test_utils::*;
        use error::CoreError;
        use super::super::parse;

        #[test]
        fn parse_simple() {
            let parsed = parse::<TestSpec>("\"simple\"").unwrap();
            assert_eq!(parsed.quoted_string, "\"simple\"");
            assert_eq!(parsed.tail, "");
        }

        #[test]
        fn parse_with_tail() {
            let parsed = parse::<TestSpec>("\"simple\"; abc").unwrap();
            assert_eq!(parsed.quoted_string, "\"simple\"");
            assert_eq!(parsed.tail, "; abc");
        }

        #[test]
        fn parse_with_quoted_pairs() {
            let parsed = parse::<TestSpec>("\"si\\\"m\\\\ple\"").unwrap();
            assert_eq!(parsed.quoted_string, "\"si\\\"m\\\\ple\"");
            assert_eq!(parsed.tail, "");
        }

        #[test]
        fn parse_with_unnecessary_quoted_pairs() {
            let parsed = parse::<TestSpec>("\"sim\\p\\le\"").unwrap();
            assert_eq!(parsed.quoted_string, "\"sim\\p\\le\"");
            assert_eq!(parsed.tail, "");
        }

        #[test]
        fn reject_missing_quoted() {
            let res = parse::<TestSpec>("simple");
            assert_eq!(res, Err((0, CoreError::DoesNotStartWithDQuotes)));
        }

        #[test]
        fn reject_tailing_escape() {
            let res = parse::<TestSpec>("\"simple\\\"");
            assert_eq!(res, Err((9, CoreError::DoesNotEndWithDQuotes)));
        }

        #[test]
        fn reject_unquoted_quotable() {
            let res = parse::<TestSpec>("\"simp\\\0le\"");
            assert_eq!(res, Err((6, CoreError::UnquoteableCharQuoted)));
        }

        #[test]
        fn reject_missing_closing_dquotes() {
            let res = parse::<TestSpec>("\"simple");
            assert_eq!(res, Err((7, CoreError::DoesNotEndWithDQuotes)));
        }

        #[test]
        fn empty_string_does_not_panic() {
            let res = parse::<TestSpec>("");
            assert_eq!(res, Err((0, CoreError::DoesNotEndWithDQuotes)));
        }

    }

    mod validate {
        use test_utils::*;
        use super::super::validate;

        #[test]
        fn accept_valid_quoted_string() {
            assert!(validate::<TestSpec>("\"that\\\"s strange\""));
        }

        #[test]
        fn reject_invalid_quoted_string() {
            assert!(!validate::<TestSpec>("ups"))
        }

        #[test]
        fn reject_quoted_string_shorter_than_input() {
            assert!(!validate::<TestSpec>("\"nice!\"ups whats here?\""))
        }

    }



}
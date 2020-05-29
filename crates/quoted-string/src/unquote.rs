use spec::{GeneralQSSpec, ScanAutomaton, PartialCodePoint};
use error::CoreError;
use std::borrow::Cow;

/// converts a quoted string into it's content
///
/// This methods retrieves the content of a quoted-string, which means it strips the
/// surrounding `'"'`-quoted, converts quoted-pairs into the values they represent and
/// strips not-semantic character.
///
/// # Example
/// ```
/// # use std::borrow::Cow;
/// //use your own Spec in practise
/// use quoted_string::test_utils::TestSpec;
/// use quoted_string::to_content;
///
/// let content = to_content::<TestSpec>("\"ab\\\"c\n\nde\"")
///     .expect("only fails if the input is not a quoted string");
/// assert_eq!(&*content, "ab\"cde");
///
/// let content = to_content::<TestSpec>("\"simple\"").unwrap();
/// // to content will just use slicing to strip `'"'`-quotes if possible
/// assert_eq!(content, Cow::Borrowed("simple"));
/// ```
///
pub fn to_content<'a, Spec: GeneralQSSpec>(
    quoted_string: &'a str
) -> Result<Cow<'a, str>, CoreError>
{
    let mut automaton = ScanAutomaton::<Spec::Parsing>::new();
    let mut continue_copy_from = None;
    for (idx, bch) in quoted_string.bytes().enumerate() {
        let emit = automaton.advance(PartialCodePoint::from_utf8_byte(bch))?;
        if !emit {
            continue_copy_from = Some(idx);
            break;
        }
    }

    if let Some(idx) = continue_copy_from {
        let mut buffer = Vec::with_capacity(quoted_string.len()-2);
        buffer.extend_from_slice(&quoted_string.as_bytes()[0..idx]);

        //SLICE_SAFE: we slice bytes so it's safe
        for bch in &quoted_string.as_bytes()[idx+1..] {
            let emit = automaton.advance(PartialCodePoint::from_utf8_byte(*bch))?;
            if emit {
                buffer.push(*bch)
            }
        }

        automaton.end()?;

        //OPTIMIZE: find a way to make sure we can't brake utf8 with emit while still being
        // byte based then use `from_utf8_unchecked`
        // Way 1: check if bch is > 127 if so make sure following UTF8_CONT's are treated the same.
        //        - needs special handling for the first non emited byte
        //        - per iter either a is `> 127` or a `& CONT_MASK == CONT_MASK` which can brake
        //          branch prediction
        //        + we can skip calling `automaton.advance(bch)` for all continuation bytes
        let strfied = String::from_utf8(buffer)
            .expect("[BUG] automaton caused a code point to be only partially emitted");

        Ok(Cow::Owned(strfied))

    } else {
        automaton.end()?;
        let len = quoted_string.len();
        Ok(Cow::Borrowed(&quoted_string[1..len-1]))
    }

}

/// strips quotes if they exists
///
/// returns None if the input does not start with `"` and ends with `"`
///
/// # Example
/// ```
/// use quoted_string::strip_dquotes;
/// assert_eq!(strip_dquotes("\"a b\""), Some("a b"));
/// assert_eq!(strip_dquotes("a b"), None);
/// assert_eq!(strip_dquotes("\"a b"), None);
/// assert_eq!(strip_dquotes("a b\""), None);
/// ```
pub fn strip_dquotes(quoted_string: &str) -> Option<&str> {
    let len = quoted_string.len();
    let bytes = quoted_string.as_bytes();
    //SLICE_SAFE: && shor circuites if len < 1 and by using bytes there is no problem with utf8
    // char boundaries
    if bytes.iter().next() == Some(&b'"') && bytes[len-1] == b'"' {
        //SLICE_SAFE: [0] and [len-1] are checked to be '"'
        Some(&quoted_string[1..len-1])
    } else {
        None
    }
}


#[cfg(test)]
mod test {

    mod to_content {
        use test_utils::*;
        use error::CoreError;
        use std::borrow::Cow;
        use super::super::to_content;

        #[test]
        fn no_quotes() {
            let res = to_content::<TestSpec>("noquotes");
            assert_eq!(res, Err(CoreError::DoesNotStartWithDQuotes));
        }

        #[test]
        fn unnecessary_quoted() {
            let res = to_content::<TestSpec>(r#""simple""#).unwrap();
            assert_eq!(res, Cow::Borrowed("simple"))
        }

        #[test]
        fn quoted_but_no_quoted_pair() {
            let res = to_content::<TestSpec>(r#""abc def""#).unwrap();
            assert_eq!(res, Cow::Borrowed("abc def"))
        }

        #[test]
        fn with_quoted_pair() {
            let res = to_content::<TestSpec>(r#""a\"b""#).unwrap();
            let expected: Cow<'static, str> = Cow::Owned(r#"a"b"#.into());
            assert_eq!(res, expected);
        }

        #[test]
        fn with_multiple_quoted_pairs() {
            let res = to_content::<TestSpec>(r#""a\"\bc\ d""#).unwrap();
            let expected: Cow<'static, str> = Cow::Owned(r#"a"bc d"#.into());
            assert_eq!(res, expected);
        }

        #[test]
        fn empty() {
            let res = to_content::<TestSpec>(r#""""#).unwrap();
            assert_eq!(res, Cow::Borrowed(""))
        }

        #[test]
        fn strip_non_semantic_ws() {
            let res = to_content::<TestSpec>("\"hy \n\nthere\"").unwrap();
            let expected: Cow<'static, str> = Cow::Owned("hy there".into());
            assert_eq!(res, expected);
        }

        #[test]
        fn tailing_escape() {
            let res = to_content::<TestSpec>(r#""ab\""#);
            assert_eq!(res, Err(CoreError::DoesNotEndWithDQuotes));
        }

        #[test]
        fn missing_escape() {
            let res = to_content::<TestSpec>("\"a\"\"");
            assert_eq!(res, Err(CoreError::QuotedStringAlreadyEnded));
        }

        #[test]
        fn custom_state_in_parsing_impl_is_used() {
            let res = to_content::<TestSpec>("\"hy \n+++---\nthere\"").unwrap();
            let expected: Cow<'static, str> = Cow::Owned("hy there".into());
            assert_eq!(res, expected);

            let res = to_content::<TestSpec>("\"hy \n+--\nthere\"");
            assert_eq!(res, Err(CoreError::InvalidChar));
        }
    }





    mod strip_quotes {
        use super::super::strip_dquotes;

        #[test]
        fn empty_string() {
            assert!(strip_dquotes("").is_none());
        }

        #[test]
        fn empty_quoted_string() {
            assert_eq!(strip_dquotes("\"\""), Some(""));
        }

        #[test]
        fn missing_quotes() {
            assert_eq!(strip_dquotes("\"abc"), None);
            assert_eq!(strip_dquotes("abc\""), None);
        }

        #[test]
        fn simple_string() {
            assert_eq!(strip_dquotes("\"simple\""), Some("simple"));
        }
    }

}

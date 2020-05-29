use std::borrow::Cow;
// this import will become unused in future rust versions
// but won't be removed for now for supporting current
// rust versions
#[allow(unused_imports, deprecated)]
use std::ascii::AsciiExt;

use error::CoreError;
use spec::{
    QuotingClassifier,
    QuotingClass,
    WithoutQuotingValidator,
    PartialCodePoint,
    GeneralQSSpec
};



/// quotes the input string returning the quoted string
///
/// # Example
///
/// ```
/// // use your own Spec instead
/// use quoted_string::test_utils::TestSpec;
/// use quoted_string::quote;
///
/// let qs = quote::<TestSpec>("some\"text").unwrap();
/// assert_eq!(qs, "\"some\\\"text\"");
/// ```
///
#[inline]
pub fn quote<Spec: GeneralQSSpec>(
    input: &str
) -> Result<String, CoreError>
{
    let mut out = String::with_capacity(input.len()+2);
    out.push('"');
    quote_inner::<Spec>(input, &mut out)?;
    out.push('"');
    Ok(out)
}

/// quotes a input writing it into the output buffer, does not add surrounding '"'
///
/// if ascii_only is true and non ascii chars a found an error is returned.
///
/// If no error is returned a boolean indicating if the whole input was ascii is
/// returned.
fn quote_inner<Spec: GeneralQSSpec>(
    input: &str,
    out: &mut String,
) -> Result<(), CoreError>
{
    use self::QuotingClass::*;
    for ch in input.chars() {
        match Spec::Quoting::classify_for_quoting(PartialCodePoint::from_code_point(ch as u32)) {
            QText => out.push(ch),
            NeedsQuoting => { out.push('\\'); out.push(ch); }
            Invalid => return Err(CoreError::InvalidChar)
        }
    }
    Ok(())
}

/// quotes the input string if needed
///
/// The `validator` decides if the value is valid without
/// quoting it, the `Spec` type decides how quoting is done if
/// needed. The `Spec` only specifies the format of quoting
/// e.g. which values are allowed in a quoted-string but
/// wether or not a string needs quoting can often depend
/// on additional factor.
///
/// Note that this implementation expects the
/// validator and spec to be in sync, i.e. what
/// is valid without quoting does not need to
/// be escaped when appearing in a quoted string.
///
/// # Example
///
/// ```
/// # use std::borrow::Cow;
/// // use your own Spec
/// use quoted_string::test_utils::{TestSpec, TestUnquotedValidator};
/// use quoted_string::quote_if_needed;
///
/// let mut without_quoting = TestUnquotedValidator::new();
/// let quoted = quote_if_needed::<TestSpec, _>("simple", &mut without_quoting)
///     .expect("only fails if input can not be represented as quoted string with used Spec");
///
/// // The used spec states a 6 character us-ascii word does not need to be represented as
/// // quoted string
/// assert_eq!(quoted, Cow::Borrowed("simple"));
///
/// let mut without_quoting = TestUnquotedValidator::new();
/// let quoted2 = quote_if_needed::<TestSpec, _>("more complex", &mut without_quoting).unwrap();
/// let expected: Cow<'static, str> = Cow::Owned("\"more complex\"".into());
/// assert_eq!(quoted2, expected);
/// ```
///
pub fn quote_if_needed<'a, Spec, WQImpl>(
    input: &'a str,
    validator: &mut WQImpl
) -> Result<Cow<'a, str>, CoreError>
    where Spec: GeneralQSSpec,
          WQImpl: WithoutQuotingValidator
{
    let mut needs_quoting_from = None;
    for (idx, ch) in input.char_indices() {
        let pcp = PartialCodePoint::from_code_point(ch as u32);
        if !validator.next(pcp) {
            needs_quoting_from = Some(idx);
            break;
        } else {
            //FIXME check if is this even enabled in the right context
            #[cfg(debug_assertions)]
            {
                use self::QuotingClass::*;
                match Spec::Quoting::classify_for_quoting(pcp) {
                    QText => {},
                    Invalid => panic!(concat!("[BUG] representable without quoted string,",
                                            "but invalid in quoted string: {}"), ch),
                    NeedsQuoting => panic!(concat!("[BUG] representable without quoted string,",
                                            "but not without escape in quoted string: {}"), ch)
                }
            }
        }
    }

    let start_quoting_from =
        if input.len() == 0 {
            0
        } else if let Some(offset) = needs_quoting_from {
            offset
        } else {
            return if validator.end() {
                Ok(Cow::Borrowed(input))
            } else {
                let mut out = String::with_capacity(input.len() + 2);
                out.push('"');
                out.push_str(input);
                out.push('"');
                Ok(Cow::Owned(out))
            };
        };


    let mut out = String::with_capacity(input.len() + 3);
    out.push('"');
    out.push_str(&input[0..start_quoting_from]);
    quote_inner::<Spec>(&input[start_quoting_from..], &mut out)?;
    out.push('"');
    Ok(Cow::Owned(out))
}


#[cfg(test)]
mod test {
    // this import will become unused in future rust versions
    // but won't be removed for now for supporting current
    // rust versions
    #[allow(warnings)]
    use std::ascii::AsciiExt;
    use test_utils::*;
    use error::CoreError;
    use super::*;

    #[test]
    fn quote_simple() {
        let data = &[
            ("this is simple", "\"this is simple\""),
            ("with quotes\"  ", "\"with quotes\\\"  \""),
            ("with slash\\  ", "\"with slash\\\\  \"")
        ];
        for &(unquoted, quoted) in data.iter() {
            let got_quoted = quote::<TestSpec>(unquoted).unwrap();
            assert_eq!(got_quoted, quoted);
        }
    }

    #[test]
    fn quote_unquotable() {
        let res = quote::<TestSpec>("→");
        assert_eq!(res.unwrap_err(), CoreError::InvalidChar);
    }

    #[test]
    fn quote_if_needed_unneeded() {
        let mut without_quoting = TestUnquotedValidator::new();
        let out= quote_if_needed::<TestSpec, _>("abcdef", &mut without_quoting).unwrap();
        assert_eq!(out, Cow::Borrowed("abcdef"));
    }

    #[test]
    fn quote_if_needed_state() {
        let mut without_quoting = TestUnquotedValidator::new();
        let out = quote_if_needed::<TestSpec, _>("abcd.e", &mut without_quoting).unwrap();
        assert_eq!(out, Cow::Borrowed("abcd.e"));
        assert_eq!(without_quoting.count, 6);
        assert_eq!(without_quoting.last_was_dot, false)
    }

    #[test]
    fn quote_if_needed_needed_because_char() {
        let mut without_quoting = TestUnquotedValidator::new();
        let out = quote_if_needed::<TestSpec, _>("ab def", &mut without_quoting).unwrap();
        let expected: Cow<'static, str> = Cow::Owned("\"ab def\"".into());
        assert_eq!(out, expected);
        assert!(without_quoting.count >= 2);
    }

    #[test]
    fn quote_if_needed_needed_because_state() {
        let mut without_quoting = TestUnquotedValidator::new();
        let out = quote_if_needed::<TestSpec, _>("abc..f", &mut without_quoting).unwrap();
        let expected: Cow<'static, str> = Cow::Owned("\"abc..f\"".into());
        assert_eq!(out, expected);
        assert!(without_quoting.count >= 4);
    }

    #[test]
    fn quote_if_needed_needed_because_end() {
        let mut without_quoting = TestUnquotedValidator::new();
        let out = quote_if_needed::<TestSpec, _>("a", &mut without_quoting).unwrap();
        let expected: Cow<'static, str> = Cow::Owned("\"a\"".into());
        assert_eq!(out, expected);
        assert!(without_quoting.count >= 1);
    }

    #[test]
    fn quote_if_needed_empty_value() {
        let mut without_quoting = TestUnquotedValidator::new();
        let out = quote_if_needed::<TestSpec, _>("", &mut without_quoting).unwrap();
        let expected: Cow<'static, str> = Cow::Owned("\"\"".into());
        assert_eq!(out, expected);
        assert_eq!(without_quoting.count, 0);
    }
}

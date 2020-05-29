use std::borrow::Cow;
use std::cmp::max;
use super::iter::ContentChars;

/// undoes quoting of given input assuming it is a valid quoted-string
///
/// # Example
/// ```
/// # use std::borrow::Cow;
/// # use quoted_string::unquote_unchecked;
/// let unquoted = unquote_unchecked(r#""simple""#);
/// assert_eq!(unquoted, Cow::Borrowed("simple"));
///
/// let unquoted = unquote_unchecked(r#""needs quoting""#);
/// assert_eq!(unquoted, Cow::Borrowed("needs quoting"));
///
/// let unquoted = unquote_unchecked(r#""less\"simple""#);
/// assert_eq!(unquoted, Cow::Borrowed(r#"less"simple"#));
/// ```
pub fn unquote_unchecked<'a>(quoted: &'a str) -> Cow<'a, str> {
    if quoted.chars().next() != Some('"') {
        Cow::Borrowed(quoted)
    } else {
        let quoted = &quoted[1..max(1, quoted.len()-1)];
        let split_idx = quoted.bytes().position(|b| b == b'\\');
        let split_idx =
            match split_idx {
                None => return Cow::Borrowed(quoted),
                Some(idx) => idx
            };
        let (unquoted, tail) = quoted.split_at(split_idx);
        let mut buffer = String::from(unquoted);
        let iter = ContentChars::from_string_unchecked(tail);
        let (min, max) = iter.size_hint();
        let min_max_diff = max.unwrap_or(min)-min;
        buffer.reserve(min+(min_max_diff >> 1));
        for ch in iter {
            buffer.push(ch)
        }
        Cow::Owned(buffer)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use super::unquote_unchecked;

    #[test]
    fn test_no_quotes() {
        let res = unquote_unchecked("simple");
        assert_eq!(res, Cow::Borrowed("simple"))
    }

    #[test]
    fn test_unnecessary_quoted() {
        let res = unquote_unchecked(r#""simple""#);
        assert_eq!(res, Cow::Borrowed("simple"))
    }

    #[test]
    fn test_quoted_but_no_quoted_pair() {
        let res = unquote_unchecked(r#""abc def""#);
        assert_eq!(res, Cow::Borrowed("abc def"))
    }

    #[test]
    fn test_with_quoted_pair() {
        let res = unquote_unchecked(r#""a\"b""#);
        let expected: Cow<'static, str> = Cow::Owned(r#"a"b"#.into());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_with_multiple_quoted_pairs() {
        let res = unquote_unchecked(r#""a\"\bc\ d""#);
        let expected: Cow<'static, str> = Cow::Owned(r#"a"bc d"#.into());
        assert_eq!(res, expected);
    }

    #[test]
    fn test_with_tailing_halve_quoted_pair() {
        let res = unquote_unchecked(r#""\""#);
        let expected: Cow<'static, str> = Cow::Owned(r#"\"#.into());
        assert_eq!(res, expected);
    }

}

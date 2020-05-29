use std::str::Chars;
use std::iter::Iterator;
use std::cmp::{ PartialEq };

use error::CoreError;
use spec::{GeneralQSSpec, ScanAutomaton, PartialCodePoint};
// this import will become unused in future rust versions
// but won't be removed for now for supporting current
// rust versions
#[allow(warnings)]
use std::ascii::AsciiExt;

/// Analogous to PartialEq, but with _ascii_ case insensitive equality
pub trait AsciiCaseInsensitiveEq<Rhs: ?Sized> {

    /// compares this instance with other with a ascii case insensitive comparsion algorithm
    ///
    /// Note that this is _ascii_ case insensitivity. Which means this will not work
    /// well/as expected if the content contain non ascii characters.
    /// E.g. the upercase of `"ß"` was `"SS"` but by know there is also a
    /// "ẞ" used in all caps writing.
    fn eq_ignore_ascii_case(&self, other: &Rhs) -> bool;
}

/// A iterator over chars of the content represented by the quoted strings (PartialEq<&str>)
///
/// It will on the fly (without extra allocation)
/// remove the surrounding quotes and unquote quoted-pairs
///
/// It implements Eq, and PartialEq with str, &str and itself. This
/// enables to compare a quoted string with a string representing the
/// content of a quoted string.
///
/// While it does not explicity checks for validity of the quoted-string it iterates over
/// it still might notice that it is invalid as such it returns results (i.e. let the
/// consumer decide how to handle the error). If you are fine with a panic if the
/// `ContentChars` iterator was created from a invalid quoted-string, then just
/// use `.map(|r| r.expect("[BUG] ContentChars was created from malformed quoted-string")`.
/// (Note that the the only 2two thinks the current implementation does not check are
///  if a char in a quoted-pair is valid (= is quotable) and it does not call `end_validation`
///  on the internally used `QuotedValidator`, but thats a implementation detail for now)
///
/// # Example
///
/// ```
/// // use your own Spec instead
/// use quoted_string::test_utils::TestSpec;
/// use quoted_string::ContentChars;
/// use quoted_string::AsciiCaseInsensitiveEq;
///
/// let quoted_string = r#""ab\"\ c""#;
/// let cc = ContentChars::<TestSpec>::from_str(quoted_string);
/// assert_eq!(cc, "ab\" c");
/// assert!(cc.eq_ignore_ascii_case("AB\" c"));
/// assert_eq!(cc.collect::<Result<Vec<_>,_>>().unwrap().as_slice(), &[ 'a', 'b', '"', ' ', 'c' ] );
///
/// ```
#[derive(Debug, Clone)]
pub struct ContentChars<'a, Impl: GeneralQSSpec> {
    inner: Chars<'a>,
    automaton: ScanAutomaton<Impl::Parsing>
}

impl<'s, Impl> ContentChars<'s, Impl>
    where Impl: GeneralQSSpec
{

    /// creates a char iterator over the content of a quoted string
    ///
    /// the quoted string is _assumed_ to be valid and not explicitely checked for validity
    /// but because of the way unquoting works a number of error can be detected
    pub fn from_str(quoted: &'s str) -> Self {
        ContentChars {
            inner: quoted.chars(),
            automaton: ScanAutomaton::<Impl::Parsing>::new()
        }
    }

    /// creates a ContentChars iterator from a str and a QuotedValidator
    ///
    /// The `partial_quoted_content` is assumed to be a valid quoted string
    /// without the surrounding `'"'`. It might not be a the complete content
    /// of a quoted string but if it isn't the q_validator is expected to have
    /// been used on a chars stripped on the left side (and no more than that).
    /// Note that it can work with using it on less then all chars but this depends
    /// on the Spec used. E.g. if any decison of the spec only depends on the current char
    /// (QuotedValidator is zero-sized) then no char had to be used with it.
    pub fn from_parts_unchecked(
        partial_quoted_content: &'s str,
        automaton: ScanAutomaton<Impl::Parsing>
    ) -> Self
    {
        let inner = partial_quoted_content.chars();
        ContentChars{ inner, automaton }
    }
}


impl<'a, Impl> Iterator for ContentChars<'a, Impl>
    where Impl: GeneralQSSpec
{
    type Item = Result<char, CoreError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ch) = self.inner.next() {
                let res = self.automaton.advance(PartialCodePoint::from_code_point(ch as u32));
                match res {
                    Err(e) => return Some(Err(e.into())),
                    Ok(true)  => return Some(Ok(ch)),
                    Ok(false) => {},
                }
            } else {
                match self.automaton.end() {
                    Err(e) => return Some(Err(e.into())),
                    Ok(()) => return None
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}


impl<'a, Spec> PartialEq<str> for ContentChars<'a, Spec>
    where Spec: GeneralQSSpec
{

    #[inline]
    fn eq(&self, other: &str) -> bool {
        iter_eq(self.clone(), other.chars().map(|ch|Ok(ch)), |l,r|l==r)
    }
}

impl<'a, 'b, Spec> PartialEq<ContentChars<'b, Spec>> for &'a str
    where Spec: GeneralQSSpec
{
    #[inline]
    fn eq(&self, other: &ContentChars<'b, Spec>) -> bool {
        *other == **self
    }
}

impl<'a, 'b, Spec> PartialEq<&'b str> for ContentChars<'a, Spec>
    where Spec: GeneralQSSpec
{
    #[inline]
    fn eq(&self, other: &&'b str) -> bool {
        self == *other
    }
}

impl<'a, 'b, Spec> PartialEq<ContentChars<'b, Spec>> for ContentChars<'a, Spec>
    where Spec: GeneralQSSpec
{
    #[inline]
    fn eq(&self, other: &ContentChars<'b, Spec>) -> bool {
        iter_eq(self.clone(), other.clone(), |l,r|l==r)
    }
}



impl<'a, Spec> AsciiCaseInsensitiveEq<str> for ContentChars<'a, Spec>
    where Spec: GeneralQSSpec
{
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        iter_eq(self.clone(), other.chars().map(|ch|Ok(ch)), |l,r| l.eq_ignore_ascii_case(&r))
    }
}

impl<'a, 'b, Spec> AsciiCaseInsensitiveEq<ContentChars<'b, Spec>> for ContentChars<'a, Spec>
    where Spec: GeneralQSSpec
{
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &ContentChars<'b, Spec>) -> bool {
        iter_eq(self.clone(), other.clone(), |l,r|l.eq_ignore_ascii_case(&r))
    }
}

impl<'a, 'b, Spec> AsciiCaseInsensitiveEq<ContentChars<'b, Spec>> for &'a str
    where Spec: GeneralQSSpec
{
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &ContentChars<'b, Spec>) -> bool {
        other == *self
    }
}



impl<'a, 'b, Spec> AsciiCaseInsensitiveEq<&'b str> for ContentChars<'a, Spec>
    where Spec: GeneralQSSpec
{
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &&'b str) -> bool {
        self == *other
    }
}

fn iter_eq<I1, I2, E, FN>(mut left: I1, mut right: I2, cmp: FN) -> bool
    where I1: Iterator<Item=Result<char, E>>,
          I2: Iterator<Item=Result<char, E>>, FN: Fn(char, char) -> bool
{
    loop {
        match (left.next(), right.next()) {
            (None, None) => return true,
            (Some(Ok(x)), Some(Ok(y))) if cmp(x, y) => (),
            _ => return false
        }
    }
}



#[cfg(test)]
mod test {
    use test_utils::*;
    use error::CoreError;
    use super::{ContentChars, AsciiCaseInsensitiveEq};

    #[test]
    fn missing_double_quoted() {
        let mut chars = ContentChars::<TestSpec>::from_str("abcdef");
        assert_eq!(chars.next().expect("is some").unwrap_err(), CoreError::DoesNotStartWithDQuotes);
    }

    #[test]
    fn unnecessary_quoted() {
        let res = ContentChars::<TestSpec>::from_str("\"abcdef\"");
        assert_eq!(res.collect::<Result<Vec<_>, _>>().unwrap().as_slice(), &[
            'a', 'b', 'c' ,'d', 'e', 'f'
        ])
    }

    #[test]
    fn quoted() {
        let res = ContentChars::<TestSpec>::from_str("\"abc def\"");
        assert_eq!(res.collect::<Result<Vec<_>, _>>().unwrap().as_slice(), &[
            'a', 'b', 'c', ' ', 'd', 'e', 'f'
        ])
    }

    #[test]
    fn with_quoted_pair() {
        let res = ContentChars::<TestSpec>::from_str(r#""abc\" \def""#);
        assert_eq!(res.collect::<Result<Vec<_>, _>>().unwrap().as_slice(), &[
            'a', 'b', 'c', '"', ' ', 'd', 'e', 'f'
        ])
    }

    #[test]
    fn strip_non_semantic_ws() {
        let res = ContentChars::<TestSpec>::from_str("\"abc\n\ndef\"");
        assert_eq!(res.collect::<Result<Vec<_>, _>>().unwrap().as_slice(), &[
            'a', 'b', 'c', 'd', 'e', 'f'
        ])
    }

    #[test]
    fn ascii_case_insensitive_eq() {
        let left = ContentChars::<TestSpec>::from_str(r#""abc""#);
        let right = ContentChars::<TestSpec>::from_str(r#""aBc""#);
        assert!(left.eq_ignore_ascii_case(&right))
    }
}

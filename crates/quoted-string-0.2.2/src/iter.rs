use std::str::Chars;
use std::iter::Iterator;
use std::cmp::{ PartialEq, Eq, max };

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
/// # Example
///
/// ```
/// # use quoted_string::ContentChars;
/// use quoted_string::AsciiCaseInsensitiveEq;
///
/// let quoted_string = r#""ab\"\ c""#;
/// let cc = ContentChars::from_string_unchecked(quoted_string);
/// assert_eq!(cc, "ab\" c");
/// assert!(cc.eq_ignore_ascii_case("AB\" c"));
/// assert_eq!(cc.collect::<Vec<_>>().as_slice(), &[ 'a', 'b', '"', ' ', 'c' ] );
///
/// ```
#[derive(Debug, Clone)]
pub struct ContentChars<'a> {
    inner: Chars<'a>
}

impl<'s> ContentChars<'s> {

    /// Crates a `ContentChars` iterator from a &str slice,
    /// assuming it represents a valid quoted-string
    ///
    /// It can be used on both with a complete quoted
    /// string and one from which the surrounding double
    /// quotes where stripped.
    ///
    /// This function relies on quoted to be valid, if it isn't
    /// it will not panic, but might not yield the expected result
    /// as there is no clear way how to handle invalid input.
    pub fn from_string_unchecked(quoted: &'s str) -> Self {
        let inner =
            if quoted.chars().next() == Some('"') {
                // do not panic on invalid input "\"" is seen as "\"\""
                quoted[1..max(1, quoted.len()-1)].chars()
            } else {
                quoted.chars()
            };

        ContentChars{ inner: inner }
    }
}


impl<'a> Iterator for ContentChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|ch| {
                if ch == '\\' {
                    // do not panic on invalid input "\"\\\"" is seen as "\"\\\\\""
                    self.inner.next().unwrap_or('\\')
                } else {
                    ch
                }
            })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}


impl<'a> PartialEq<str> for ContentChars<'a> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        iter_eq(self.clone(), other.chars())
    }
}

impl<'a, 'b> PartialEq<ContentChars<'b>> for &'a str {
    #[inline]
    fn eq(&self, other: &ContentChars<'b>) -> bool {
        iter_eq(self.chars(), other.clone())
    }
}

impl<'a, 'b> PartialEq<&'b str> for ContentChars<'a> {
    #[inline]
    fn eq(&self, other: &&'b str) -> bool {
        iter_eq(self.clone(), other.chars())
    }
}

impl<'a, 'b> PartialEq<ContentChars<'b>> for ContentChars<'a> {
    #[inline]
    fn eq(&self, other: &ContentChars<'b>) -> bool {
        iter_eq(self.clone(), other.clone())
    }
}

impl<'a> Eq for ContentChars<'a> {}



impl<'a> AsciiCaseInsensitiveEq<str> for ContentChars<'a> {
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &str) -> bool {
        iter_eq_ascii_case_insensitive(self.clone(), other.chars())
    }
}

impl<'a, 'b> AsciiCaseInsensitiveEq<ContentChars<'b>> for ContentChars<'a> {
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &ContentChars<'b>) -> bool {
        iter_eq_ascii_case_insensitive(self.clone(), other.clone())
    }
}

impl<'a, 'b> AsciiCaseInsensitiveEq<ContentChars<'b>> for &'a str {
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &ContentChars<'b>) -> bool {
        iter_eq_ascii_case_insensitive(self.chars(), other.clone())
    }
}



impl<'a, 'b> AsciiCaseInsensitiveEq<&'b str> for ContentChars<'a>  {
    #[inline]
    fn eq_ignore_ascii_case(&self, other: &&'b str) -> bool {
        iter_eq_ascii_case_insensitive(self.clone(), other.chars())
    }
}

fn iter_eq<I1, I2>(mut left: I1, mut right: I2) -> bool
    where I1: Iterator<Item=char>,
          I2: Iterator<Item=char>
{
    loop {
        match (left.next(), right.next()) {
            (None, None) => break,
            (Some(x), Some(y)) if x == y => (),
            _ => return false
        }
    }
    true
}

fn iter_eq_ascii_case_insensitive<I1, I2>(mut left: I1, mut right: I2) -> bool
    where I1: Iterator<Item=char>,
          I2: Iterator<Item=char>
{
    loop {
        match (left.next(), right.next()) {
            (None, None) => break,
            (Some(x), Some(y)) if x.eq_ignore_ascii_case(&y) => (),
            _ => return false
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::{ContentChars, AsciiCaseInsensitiveEq};

    #[test]
    fn no_quotation() {
        let res = ContentChars::from_string_unchecked("abcdef");
        assert_eq!(res.collect::<Vec<_>>().as_slice(), &[
            'a', 'b', 'c' ,'d', 'e', 'f'
        ])
    }

    #[test]
    fn unnecessary_quoted() {
        let res = ContentChars::from_string_unchecked("\"abcdef\"");
        assert_eq!(res.collect::<Vec<_>>().as_slice(), &[
            'a', 'b', 'c' ,'d', 'e', 'f'
        ])
    }

    #[test]
    fn quoted() {
        let res = ContentChars::from_string_unchecked("\"abc def\"");
        assert_eq!(res.collect::<Vec<_>>().as_slice(), &[
            'a', 'b', 'c', ' ', 'd', 'e', 'f'
        ])
    }

    #[test]
    fn with_quoted_pair() {
        let res = ContentChars::from_string_unchecked(r#""abc\" \def""#);
        assert_eq!(res.collect::<Vec<_>>().as_slice(), &[
            'a', 'b', 'c', '"', ' ', 'd', 'e', 'f'
        ])
    }

    #[test]
    fn missing_double_quoted() {
        let res = ContentChars::from_string_unchecked(r#"abc\" \def"#);
        assert_eq!(res.collect::<Vec<_>>().as_slice(), &[
            'a', 'b', 'c', '"', ' ', 'd', 'e', 'f'
        ])
    }

    #[test]
    fn ascii_case_insensitive_eq() {
        let left = ContentChars::from_string_unchecked(r#""abc""#);
        let right = ContentChars::from_string_unchecked(r#""aBc""#);
        assert!(left.eq_ignore_ascii_case(&right))
    }
}

quoted-string [![Crates.io](https://img.shields.io/crates/v/quoted-string.svg)](https://crates.io/crates/quoted-string) [![quoted-string](https://docs.rs/quoted-string/badge.svg)](https://docs.rs/quoted-string) [![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Build Status](https://travis-ci.org/1aim/quoted-string.svg?branch=master)](https://travis-ci.org/1aim/quoted-string)
=============

This crate provides utilities to handle quoted strings like such appearing
in Media Types (both MIME (i.e. Mail) and HTTP). As there are many small but significant
 differences in different specifications this crate does not provide
a specific implementation. Instead a `QuotedStringSpec` trait is
exposed. Implementing it (on zero-sized types) should allow the
usage with any quoted-string specification.

Available functionality contains
--------------------------------

- `quote_if_needed` (&`quote`): quotes content (if needed), the `UnquotedValidator`
  part of `QuotedStringSpec` can be used to specify which values are valid without
  needing to be represented as a quoted string. E.g. in a Media Type a parameter
  value of `abc` can and should be directly represented on benefit `quoted_if_needed`
  is that it returns a `Cow` so the string is only copied if it actually needs to be
  represented as quoted string.

- `to_content`: retrieve the _content_ of a quoted string which means that the
  surrounding `'"'` quotes will be removed and any quoted-pair (e.g. `"\\\""`/`r#"\""#`)
  will be replaced with it's value. This function returns `Cow::Borrowed` if there are
  no quoted-pairs preventing needless allocations.

- `ContentChars`: an iterator over the chars of the content of a quoted-string,
  i.e. it will strip the surrounding `DQUOTE`s and will ()on the fly) unquote
  quoted-pairs not needing any extra memory allocations. This can be used to
  _semantically_ compare two quoted strings regardless of how they used
  `quoted-pair`s, it implements `Eq`.

- `parse` (&`validate`):  parses a quoted-string positioned at the start of the input.
  It is written to be easily integrateable with `nom` (through does not require `nom`
  in any way, using it standalone is as easy)

Example
-------

```rust
extern crate quoted_string;

// we use a QuotedStringSpec provided for testing here,
// not that it's made to hit some edge cases in a simple way
// so it does not correspond to any used real Spec
use quoted_string::test_utils::{TestSpec as Spec};
use quoted_string::spec::AsciiWordValidator;
use quoted_string::{parse, quote, quote_if_needed, to_content};

fn main() {
    let res = parse::<Spec>("\"quoted\\\"st\\ring\"; tail=x").unwrap();
    let qs = res.quoted_string;
    assert_eq!(qs, "\"quoted\\\"st\\ring\"");
    assert_eq!(res.tail, "; tail=x");
    let content = to_content::<Spec>(qs)
        .expect("[BUG] to_content is guaranteed to succeed if input is a valid quoted string");
    assert_eq!(content, "quoted\"string");
    let re_quoted = quote::<Spec>(&*content)
        .expect("[BUG] quote is guaranteed to succeed if the input is representable in a quoted string");
    assert_eq!(re_quoted, "\"quoted\\\"string\"");

    // TestSpec specifies us-ascii words with 6 letters need no quoting
    let mut without_quoting = AsciiWordValidator;
    let out = quote_if_needed::<Spec, _>("simple", &mut without_quoting).unwrap();
    assert_eq!(&*out, "simple");
}


```

License
--------

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

Change Log
----------

- **0.3.0**: Made the crate independent of any specific quoted-string specification by
  introducing `QuotedStringSpec` as there are to many differences between quoted-string's
  in Media Type occurring in HTTP and thus occurring in MIME (Mail)

- **0.3.1**: Noticed that `ValidationResult` was not exposed, but also
  didn't got a private type in public interface warning... fixed that

- **0.4.0**:
  - Removed `Escape` from `ValidationResult`
  - Renamed `NotSemanticWs` to `NotSemantic`
  - Renamed `Quotable` to `NeedsQuotedPair`
  - Removed unnecessary `Err` Associated Type from `UnquotedValidator`
  - added default impl for `end_validation` of `UnquotedValidator` and`QuotedValidator`

- **0.5.0**:
  - Changed Spec to use a automaton internally
  - renamed `strip_quotes` to `strip_dquotes`
  - default impl for WithoutQuotingValidator::end

- **0.6.0**:
  - min rust version is now `rustc v1.24`
  - added `AsciiWordValidator`
  - fixed example in README
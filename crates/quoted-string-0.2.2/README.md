quoted-string [![Crates.io](https://img.shields.io/crates/v/quoted-string.svg)](https://crates.io/crates/quoted-string) [![quoted-string](https://docs.rs/quoted-string/badge.svg)](https://docs.rs/quoted-string) [![License](https://img.shields.io/badge/License-MIT%2FApache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![Build Status](https://travis-ci.org/1aim/quoted-string.svg?branch=master)](https://travis-ci.org/1aim/quoted-string)
=============
This crate provides utilities to handle quoted strings as they appear in
multiple mail and web related RFCs. While it is mainly based on RFC5322
(Internet Message Format).  It also supports Utf8 based on RFC6532 (optional)
and is compatible with quoted-strings as they appear in mime types (including
in HTTP/1.1 context).

What it currently does not support are soft-line breaks of RFC5322 and the
obsolete parts of the syntax.

Grammar
-------
```no-rust
quoted-string   = DQUOTE *( *WSP qcontent) *WSP DQUOTE
WSP = ' ' / '\t'
qcontent = qtext / quoted-pair
qtext = %d33 / %d35-91 / %d93-126 ; printable us-ascii chars not including '\\' and '"'
quoted-pair = ("\" (VCHAR / WSP)) ; VCHAR are printable us-ascii chars
```

The obsolete syntax is currently **not supported**. Differences would be:

1. it would allow CTL's in qtext
2. it would allow quoted pairs to escape CTL's, `'\0'`, `'\n'`, `'\r'` 
   
Nevertheless this part of the syntax is obsolete and should not be generated at
all. Adding opt-in support for parts parsing quoted-string is in consideration. 

Available functionality contains
--------------------------------
- `quote`/`quote_if_needed`: quotes content (if needed) allowing the usage of a
  custom context to define when quoting is needed (e.g. in some places a empty
  quoted-string is ok but no empty unquoted-string).  For optimization
  `quote_if_needed` returns a `Cow<'a, str>`.

- `unquote`: retrieve the content of a quoted string by unquoting it, also
  returns a `Cow<'a, str>` as simple string slicing can be used as long as no
  `quoted-pair` appears.
 
- `ContentChars`: an iterator over the chars of the content of a quoted-string,
  i.e. it will strip the surrounding `DQUOTE`s and will on the fly unquote
  quoted-pais not needing any extra memory allocations. This can be used to
  _semantically_ compare two quoted strings regardless of how they used
  `quoted-pair`s, it implements `Eq`.
  
- `CharType`: classifies chars using a lookup table, mainly used to determine
  which chars can be represented in a quoted-string and how, but can also be
  used to e.g. determine if a string is a valid token (RFC2045)

License
=======
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Contribution
------------
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

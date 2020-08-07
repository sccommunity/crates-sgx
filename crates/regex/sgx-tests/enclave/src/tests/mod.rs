

mod macros;
mod macros_bytes;
mod macros_str;
mod test_default;
#[macro_use]
mod test_default_bytes;
#[macro_use]
mod api;
//mod api_str;
mod crazy;
mod flags;
mod fowler;
// #[macro_use]
// mod replace;
// mod searcher;
// #[macro_use]
// mod multiline;
// mod noparse;
// mod regression;

mod set;
mod suffix_reverse;
#[cfg(feature = "unicode")]
mod unicode;
#[cfg(feature = "unicode-perl")]
mod word_boundary;
#[cfg(feature = "unicode-perl")]
mod word_boundary_unicode;

mod consistent;


//mod test_crates_regex;



mod test_backtrack_bytes;

mod test_backtrack_utf8bytes;

mod test_backtrack;



mod test_nfa_bytes;

mod test_nfa_utf8bytes;

mod test_nfa;

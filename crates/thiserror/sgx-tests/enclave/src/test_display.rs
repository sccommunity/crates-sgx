use std::prelude::v1::*;
use std::format;
use std::fmt::Display;
use thiserror::Error;

fn assert<T: Display>(expected: &str, value: T) {
    assert_eq!(expected, value.to_string());
}

fn test_braced() {
    #[derive(Error, Debug)]
    #[error("braced error: {msg}")]
    struct Error {
        msg: String,
    }

    let msg = "T".to_owned();
    assert("braced error: T", Error { msg });
}

fn test_braced_unused() {
    #[derive(Error, Debug)]
    #[error("braced error")]
    struct Error {
        extra: usize,
    }

    assert("braced error", Error { extra: 0 });
}

fn test_tuple() {
    #[derive(Error, Debug)]
    #[error("tuple error: {0}")]
    struct Error(usize);

    assert("tuple error: 0", Error(0));
}

fn test_unit() {
    #[derive(Error, Debug)]
    #[error("unit error")]
    struct Error;

    assert("unit error", Error);
}

fn test_enum() {
    #[derive(Error, Debug)]
    enum Error {
        #[error("braced error: {id}")]
        Braced { id: usize },
        #[error("tuple error: {0}")]
        Tuple(usize),
        #[error("unit error")]
        Unit,
    }

    assert("braced error: 0", Error::Braced { id: 0 });
    assert("tuple error: 0", Error::Tuple(0));
    assert("unit error", Error::Unit);
}

fn test_constants() {
    #[derive(Error, Debug)]
    #[error("{MSG}: {id:?} (code {CODE:?})")]
    struct Error {
        id: &'static str,
    }

    const MSG: &str = "failed to do";
    const CODE: usize = 9;

    assert("failed to do: \"\" (code 9)", Error { id: "" });
}

fn test_inherit() {
    #[derive(Error, Debug)]
    #[error("{0}")]
    enum Error {
        Some(&'static str),
        #[error("other error")]
        Other(&'static str),
    }

    assert("some error", Error::Some("some error"));
    assert("other error", Error::Other("..."));
}

fn test_brace_escape() {
    #[derive(Error, Debug)]
    #[error("fn main() {{}}")]
    struct Error;

    assert("fn main() {}", Error);
}

fn test_expr() {
    #[derive(Error, Debug)]
    #[error("1 + 1 = {}", 1 + 1)]
    struct Error;
    assert("1 + 1 = 2", Error);
}

fn test_nested() {
    #[derive(Error, Debug)]
    #[error("!bool = {}", not(.0))]
    struct Error(bool);

    fn not(bool: &bool) -> bool {
        !*bool
    }

    assert("!bool = false", Error(true));
}

fn test_match() {
    #[derive(Error, Debug)]
    #[error("{}: {0}", match .1 {
        Some(n) => format!("error occurred with {}", n),
        None => format!("there was an empty error"),
    })]
    struct Error(String, Option<usize>);

    assert(
        "error occurred with 1: ...",
        Error("...".to_owned(), Some(1)),
    );
    assert(
        "there was an empty error: ...",
        Error("...".to_owned(), None),
    );
}

fn test_void() {
    #[derive(Error, Debug)]
    #[error("...")]
    pub enum Error {}
}

fn test_mixed() {
    #[derive(Error, Debug)]
    #[error("a={a} :: b={} :: c={c} :: d={d}", 1, c = 2, d = 3)]
    struct Error {
        a: usize,
        d: usize,
    }

    assert("a=0 :: b=1 :: c=2 :: d=3", Error { a: 0, d: 0 });
}

fn test_ints() {
    #[derive(Error, Debug)]
    enum Error {
        #[error("error {0}")]
        Tuple(usize, usize),
        #[error("error {0}", '?')]
        Struct { v: usize },
    }

    assert("error 9", Error::Tuple(9, 0));
    assert("error ?", Error::Struct { v: 0 });
}

fn test_trailing_comma() {
    #[derive(Error, Debug)]
    #[error(
        "error {0}",
    )]
    #[rustfmt::skip]
    struct Error(char);

    assert("error ?", Error('?'));
}

fn test_field() {
    #[derive(Debug)]
    struct Inner {
        data: usize,
    }

    #[derive(Error, Debug)]
    #[error("{}", .0.data)]
    struct Error(Inner);

    assert("0", Error(Inner { data: 0 }));
}

pub fn run_tests() {
    use sgx_tunittest::*;
    rsgx_unit_tests!(
        test_braced,
        test_braced_unused,
        test_tuple,
        test_unit,
        test_enum,
        test_constants,
        test_inherit,
        test_brace_escape,
        test_expr,
        test_nested,
        test_match,
        test_void,
        test_mixed,
        test_ints,
        test_trailing_comma,
        test_field,
    );
}

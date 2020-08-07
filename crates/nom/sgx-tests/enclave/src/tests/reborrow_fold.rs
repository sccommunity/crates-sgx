#![allow(dead_code)]
#![allow(unused_variables)]


use std::str;
use std::prelude::v1::*;
named_args!(atom<'a>(tomb: &'a mut ())<String>,
            map!(map_res!(is_not!(" \t\r\n()"), str::from_utf8), ToString::to_string));

/*FIXME: should we support the use case of borrowing data mutably in a parser?
named_args!(list<'a>(tomb: &'a mut ())<String>,
  delimited!(
    char!('('),
    fold_many0!(call!(atom, tomb), "".to_string(), |acc: String, next: String| acc + next.as_str()),
    char!(')')));
*/

mod smoke;
use std::prelude::v1::*;
use std::string::ToString;
use crates_unittest::run_inventory_tests;

pub fn run_tests() {
    run_inventory_tests!();
}
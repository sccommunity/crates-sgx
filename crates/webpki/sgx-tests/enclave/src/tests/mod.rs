mod integration;
mod dns_name_tests;
use std::prelude::v1::*;
use crates_unittest::run_inventory_tests;

pub fn run_tests() {
    run_inventory_tests!();
}
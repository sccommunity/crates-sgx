mod channel;
mod mpsc_close;
mod mpsc;
mod oneshot;
use std::prelude::v1::*;

use crates_unittest::run_inventory_tests;

pub fn run_tests() {
    run_inventory_tests!();
}
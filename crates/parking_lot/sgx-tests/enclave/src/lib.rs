#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use parking_lot::RwLock;
use std::thread;

struct Bar(RwLock<()>);

impl Drop for Bar {
    fn drop(&mut self) {
        let _n = self.0.write();
    }
}

thread_local! {
    static B: Bar = Bar(RwLock::new(()));
}

fn run_tests_issue_203() {
    thread::spawn(|| {
        B.with(|_| ());

        let a = RwLock::new(());
        let _a = a.read();
    })
    .join()
    .unwrap();
}

#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    run_tests_issue_203();
}

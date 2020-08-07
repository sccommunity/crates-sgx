//extern crate autocfg;

fn main() {
    //let ac = autocfg::new();
    //ac.emit_sysroot_crate("std");
    //autocfg::rerun_path(file!());
    println!("cargo:rustc-cfg=has_std");
}

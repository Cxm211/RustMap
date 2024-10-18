#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {}
fn main_0() -> i32 {
    let mut r: libc::c_float = 7.125f64 as libc::c_float;
    print!(" {:9.3}\n", -r as f64);
    print!(" {:9.3}\n", r as f64);
    print!(" {:-9.3}\n", r as f64);
    print!(" {:09.3}\n", -r as f64);
    print!(" {:09.3}\n", r as f64);
    print!(" {:-09.3}\n", r as f64);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

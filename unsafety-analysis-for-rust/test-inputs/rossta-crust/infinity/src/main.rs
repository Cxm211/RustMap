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
#[no_mangle]
pub extern "C" fn inf() -> f64 {
    return ::core::f64::INFINITY;
}

fn main_0() -> i32 {
    print!("{}\n", inf());
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

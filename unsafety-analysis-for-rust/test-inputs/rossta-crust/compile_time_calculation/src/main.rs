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
pub static mut val: i32 = 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10;
fn main_0() -> i32 {
    unsafe {
        print!("10! = {}\n", val);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

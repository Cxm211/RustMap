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
    let mut i: i32 = 0;
    i = 1;
    while i <= 10 {
        print!("{}", i);
        if i == 10 {
            print!("\n")
        } else {
            print!(", ")
        };
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

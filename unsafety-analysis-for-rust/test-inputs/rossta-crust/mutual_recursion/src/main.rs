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
pub extern "C" fn F(n: i32) -> i32 {
    return if n == 0 { 1 } else { n - M(F(n - 1)) };
}

#[no_mangle]
pub extern "C" fn M(n: i32) -> i32 {
    return if n == 0 { 0 } else { n - F(M(n - 1)) };
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < 20 {
        print!("{:2} ", F(i));
        i += 1;
        i;
    }
    print!("\n");
    i = 0;
    while i < 20 {
        print!("{:2} ", M(i));
        i += 1;
        i;
    }
    print!("\n");
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

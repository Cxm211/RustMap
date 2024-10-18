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
pub extern "C" fn a(mut in_0: bool) -> bool {
    print!("I am a\n");
    return in_0;
}

#[no_mangle]
pub extern "C" fn b(mut in_0: bool) -> bool {
    print!("I am b\n");
    return in_0;
}

fn main_0() -> i32 {
    let mut x: bool = false;
    x = a(0 != 0) as i32 != 0 && b(1 != 0) as i32 != 0;
    if x as i32 != 0 {
        print!("false && true = {}\n\n", "true\0")
    } else {
        print!("false && true = {}\n\n", "false\0")
    };
    x = a(1 != 0) as i32 != 0 || b(0 != 0) as i32 != 0;
    if x as i32 != 0 {
        print!("true || false = {}\n\n", "true\0")
    } else {
        print!("true || false = {}\n\n", "false\0")
    };
    x = a(1 != 0) as i32 != 0 && b(0 != 0) as i32 != 0;
    if x as i32 != 0 {
        print!("true && false = {}\n\n", "true\0")
    } else {
        print!("true && false = {}\n\n", "false\0")
    };
    x = a(0 != 0) as i32 != 0 || b(0 != 0) as i32 != 0;
    if x as i32 != 0 {
        print!("false || false = {}\n\n", "true\0")
    } else {
        print!("false || false = {}\n\n", "false\0")
    };
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
pub extern "C" fn mul_inv(mut a: i32, mut b: i32) -> i32 {
    let mut b0: i32 = b;
    let mut t: i32 = 0;
    let mut q: i32 = 0;
    let mut x0: i32 = 0;
    let mut x1: i32 = 1;
    if b == 1 {
        return 1;
    }
    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 += b0;
    }
    return x1;
}

fn main_0() -> i32 {
    print!("{}\n", mul_inv(42, 2017));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

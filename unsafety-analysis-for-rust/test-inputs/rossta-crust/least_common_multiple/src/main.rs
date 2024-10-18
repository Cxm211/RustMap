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
pub extern "C" fn gcd(mut m: i32, mut n: i32) -> i32 {
    let mut tmp: i32 = 0;
    while m != 0 {
        tmp = m;
        m = n % m;
        n = tmp;
    }
    return n;
}

#[no_mangle]
pub extern "C" fn lcm(mut m: i32, mut n: i32) -> i32 {
    return m / gcd(m, n) * n;
}

fn main_0() -> i32 {
    print!("lcm(35, 21) = {}\n", lcm(21, 35));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

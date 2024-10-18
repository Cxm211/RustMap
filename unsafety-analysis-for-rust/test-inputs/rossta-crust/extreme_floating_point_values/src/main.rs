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
    let mut inf: f64 = 1 as f64 / 0.0f64;
    let mut minus_inf: f64 = -1i32 as f64 / 0.0f64;
    let mut minus_zero: f64 = -1i32 as f64 / inf;
    let mut nan: f64 = 0.0f64 / 0.0f64;
    print!("positive infinity: {}\n", inf);
    print!("negative infinity: {}\n", minus_inf);
    print!("negative zero: {}\n", minus_zero);
    print!("not a number: {}\n", nan);
    print!("+inf + 2.0 = {}\n", inf + 2.0f64);
    print!("+inf - 10.1 = {}\n", inf - 10.1f64);
    print!("+inf + -inf = {}\n", inf + minus_inf);
    print!("0.0 * +inf = {}\n", 0.0f64 * inf);
    print!("1.0/-0.0 = {}\n", 1.0f64 / minus_zero);
    print!("NaN + 1.0 = {}\n", nan + 1.0f64);
    print!("NaN + NaN = {}\n", nan + nan);
    if nan == nan {
        print!("NaN == NaN = {}\n", "true\0")
    } else {
        print!("NaN == NaN = {}\n", "false\0")
    };
    if 0.0f64 == minus_zero {
        print!("0.0 == -0.0 = {}\n", "true\0")
    } else {
        print!("0.0 == -0.0 = {}\n", "false\0")
    };
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

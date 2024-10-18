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
extern "C" {
    fn pow(_: f64, _: f64) -> f64;
}
fn main_0() -> i32 {
    let mut i: i32 = 1;
    unsafe {
        while i < 5000 {
            let mut sum: i32 = 0;
            let mut number: i32 = i;
            while number > 0 {
                let mut digit: i32 = number % 10;
                sum = (sum as f64 + pow(digit as f64, digit as f64)) as i32;
                number /= 10;
            }
            if sum == i {
                print!("{}\n", i);
            }
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

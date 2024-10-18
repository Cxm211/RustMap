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
    let N: i32 = 2;
    let mut base: i32 = 10;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut k: i32 = 0;
    k = 1;
    unsafe {
        while (k as f64) < pow(base as f64, N as f64) {
            c1 += 1;
            c1;
            if k % (base - 1) == k * k % (base - 1) {
                c2 += 1;
                c2;
                print!("{} ", k);
            }
            k += 1;
            k;
        }
    }
    print!(
        "\nTring {} numbers instead of {} numbers saves {}%\n",
        c2,
        c1,
        100.0f64 - 100.0f64 * c2 as f64 / c1 as f64
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

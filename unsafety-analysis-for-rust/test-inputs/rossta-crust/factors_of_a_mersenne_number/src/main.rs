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
pub extern "C" fn isPrime(mut n: i32) -> i32 {
    if n % 2 == 0 {
        return (n == 2) as i32;
    }
    if n % 3 == 0 {
        return (n == 3) as i32;
    }
    let mut d: i32 = 5;
    while d * d <= n {
        if n % d == 0 {
            return 0;
        }
        d += 2;
        if n % d == 0 {
            return 0;
        }
        d += 4;
    }
    return 1;
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut d: i32 = 0;
    let mut p: i32 = 0;
    let mut r: i32 = 0;
    let mut q: i32 = 929;
    if isPrime(q) == 0 {
        return 1;
    }
    r = q;
    while r > 0 {
        r <<= 1;
    }
    d = 2 * q + 1;
    loop {
        p = r;
        i = 1;
        while p != 0 {
            i = (i as i64 * i as i64 % d as i64) as i32;
            if p < 0 {
                i *= 2;
            }
            if i > d {
                i -= d;
            }
            p <<= 1;
        }
        if !(i != 1) {
            break;
        }
        d += 2 * q;
    }
    print!("2^{} - 1 = 0 (mod {})\n", q, d);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

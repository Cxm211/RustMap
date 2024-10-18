#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn isPrime(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    if n % 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return (n == 2 as std::os::raw::c_int) as std::os::raw::c_int;
    }
    if n % 3 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return (n == 3 as std::os::raw::c_int) as std::os::raw::c_int;
    }
    let mut d: std::os::raw::c_int = 5 as std::os::raw::c_int;
    while d * d <= n {
        if n % d == 0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int;
        }
        d += 2 as std::os::raw::c_int;
        if n % d == 0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int;
        }
        d += 4 as std::os::raw::c_int;
    }
    return 1 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = 0;
    let mut p: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut q: std::os::raw::c_int = 929 as std::os::raw::c_int;
    if isPrime(q) == 0 {
        return 1 as std::os::raw::c_int;
    }
    r = q;
    while r > 0 as std::os::raw::c_int {
        r <<= 1 as std::os::raw::c_int;
    }
    d = 2 as std::os::raw::c_int * q + 1 as std::os::raw::c_int;
    loop {
        p = r;
        i = 1 as std::os::raw::c_int;
        while p != 0 {
            i = (i as std::os::raw::c_longlong * i as std::os::raw::c_longlong % d as std::os::raw::c_longlong)
                as std::os::raw::c_int;
            if p < 0 as std::os::raw::c_int {
                i *= 2 as std::os::raw::c_int;
            }
            if i > d {
                i -= d;
            }
            p <<= 1 as std::os::raw::c_int;
        }
        if !(i != 1 as std::os::raw::c_int) {
            break;
        }
        d += 2 as std::os::raw::c_int * q;
    }
    printf(b"2^%d - 1 = 0 (mod %d)\n\0" as *const u8 as *const std::os::raw::c_char, q, d);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

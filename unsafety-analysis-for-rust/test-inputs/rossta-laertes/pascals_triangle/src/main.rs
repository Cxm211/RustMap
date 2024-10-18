#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub static mut N: std::os::raw::c_int = 15 as std::os::raw::c_int;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut k: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut num: std::os::raw::c_ulonglong = 0;
    let mut den: std::os::raw::c_ulonglong = 0;
    let mut catalan: std::os::raw::c_int = 0;
    printf(b"1 \0" as *const u8 as *const std::os::raw::c_char);
    n = 2 as std::os::raw::c_int;
    while n <= N {
        den = 1 as std::os::raw::c_int as std::os::raw::c_ulonglong;
        num = den;
        k = 2 as std::os::raw::c_int;
        while k <= n {
            num = num.wrapping_mul((n + k) as std::os::raw::c_ulonglong);
            den = den.wrapping_mul(k as std::os::raw::c_ulonglong);
            catalan = num.wrapping_div(den) as std::os::raw::c_int;
            k += 1;
            k;
        }
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, catalan);
        n += 1;
        n;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let N: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut base: std::os::raw::c_int = 10 as std::os::raw::c_int;
    let mut c1: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut c2: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut k: std::os::raw::c_int = 0;
    k = 1 as std::os::raw::c_int;
    while (k as std::os::raw::c_double) < pow(base as std::os::raw::c_double, N as std::os::raw::c_double) {
        c1 += 1;
        c1;
        if k % (base - 1 as std::os::raw::c_int) == k * k % (base - 1 as std::os::raw::c_int) {
            c2 += 1;
            c2;
            printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, k);
        }
        k += 1;
        k;
    }
    printf(
        b"\nTring %d numbers instead of %d numbers saves %f%%\n\0" as *const u8
            as *const std::os::raw::c_char,
        c2,
        c1,
        100.0f64 - 100.0f64 * c2 as std::os::raw::c_double / c1 as std::os::raw::c_double,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

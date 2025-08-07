#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mul_inv(mut a: std::os::raw::c_int, mut b: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut b0: std::os::raw::c_int = b;
    let mut t: std::os::raw::c_int = 0;
    let mut q: std::os::raw::c_int = 0;
    let mut x0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut x1: std::os::raw::c_int = 1 as std::os::raw::c_int;
    if b == 1 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int;
    }
    while a > 1 as std::os::raw::c_int {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 as std::os::raw::c_int {
        x1 += b0;
    }
    return x1;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        mul_inv(42 as std::os::raw::c_int, 2017 as std::os::raw::c_int),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

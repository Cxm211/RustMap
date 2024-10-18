#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut u8,
    pub reg_save_area: *mut u8,
}
pub type va_list = __builtin_va_list;
#[no_mangle]
pub extern "C" fn factorial(mut n: std::os::raw::c_int) -> std::os::raw::c_long {
    if n > 1 as std::os::raw::c_int {
        return n as std::os::raw::c_long * factorial(n - 1 as std::os::raw::c_int);
    }
    return 1 as std::os::raw::c_int as std::os::raw::c_long;
}
#[no_mangle]
pub extern "C" fn sumOfFactorials(
    mut num: std::os::raw::c_int,
    mut args: ...
) -> std::os::raw::c_long {
    let mut vaList: std::ffi::VaListImpl;
    let mut sum: std::os::raw::c_long = 0 as std::os::raw::c_int as std::os::raw::c_long;
    vaList = args.clone();
    loop {
        let fresh0 = num;
        num = num - 1;
        if !(fresh0 != 0) {
            break;
        }
        sum += factorial(vaList.arg::<std::os::raw::c_int>());
    }
    return sum;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"\nSum of factorials of [1,5] : %ld\0" as *const u8 as *const std::os::raw::c_char,
        sumOfFactorials(
            5 as std::os::raw::c_int,
            1 as std::os::raw::c_int,
            2 as std::os::raw::c_int,
            3 as std::os::raw::c_int,
            4 as std::os::raw::c_int,
            5 as std::os::raw::c_int,
        ),
    );
    printf(
        b"\nSum of factorials of [3,5] : %ld\0" as *const u8 as *const std::os::raw::c_char,
        sumOfFactorials(
            3 as std::os::raw::c_int,
            3 as std::os::raw::c_int,
            4 as std::os::raw::c_int,
            5 as std::os::raw::c_int,
        ),
    );
    printf(
        b"\nSum of factorials of [1,3] : %ld\0" as *const u8 as *const std::os::raw::c_char,
        sumOfFactorials(
            3 as std::os::raw::c_int,
            1 as std::os::raw::c_int,
            2 as std::os::raw::c_int,
            3 as std::os::raw::c_int,
        ),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

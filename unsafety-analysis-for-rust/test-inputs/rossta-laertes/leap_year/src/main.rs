#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn is_leap_year(mut year: std::os::raw::c_int) -> std::os::raw::c_int {
    return if year % 4 as std::os::raw::c_int == 0 && year % 100 as std::os::raw::c_int != 0
        || year % 400 as std::os::raw::c_int == 0
    {
        1 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut test_case: [std::os::raw::c_int; 5] = [
        1900 as std::os::raw::c_int,
        1994 as std::os::raw::c_int,
        1996 as std::os::raw::c_int,
        1997 as std::os::raw::c_int,
        2000 as std::os::raw::c_int,
    ];
    let mut key: std::os::raw::c_int = 0;
    let mut end: std::os::raw::c_int = 0;
    let mut year: std::os::raw::c_int = 0;
    key = 0 as std::os::raw::c_int;
    end = (std::mem::size_of::<[std::os::raw::c_int; 5]>() as std::os::raw::c_ulong)
        .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    while key < end {
        year = test_case[key as usize];
        printf(
            b"%d is %sa leap year.\n\0" as *const u8 as *const std::os::raw::c_char,
            year,
            if is_leap_year(year) == 1 as std::os::raw::c_int {
                b"\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"not \0" as *const u8 as *const std::os::raw::c_char
            },
        );
        key += 1;
        key;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

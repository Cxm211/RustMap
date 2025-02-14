#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fusc(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    if n == 0 as std::os::raw::c_int || n == 1 as std::os::raw::c_int {
        return n
    } else if n % 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return fusc(n / 2 as std::os::raw::c_int)
    } else {
        return fusc((n - 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int)
            + fusc((n + 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn numLen(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut sum: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while n > 9 as std::os::raw::c_int {
        n = n / 10 as std::os::raw::c_int;
        sum += 1;
        sum;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn printLargeFuscs(mut limit: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut f: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut maxLen: std::os::raw::c_int = 1 as std::os::raw::c_int;
    printf(
        b"\n\nPrinting all largest Fusc numbers upto %d \nIndex-------Value\0"
            as *const u8 as *const std::os::raw::c_char,
        limit,
    );
    i = 0 as std::os::raw::c_int;
    while i <= limit {
        f = fusc(i);
        len = numLen(f);
        if len > maxLen {
            maxLen = len;
            printf(b"\n%5d%12d\0" as *const u8 as *const std::os::raw::c_char, i, f);
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    printf(b"Index-------Value\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < 61 as std::os::raw::c_int {
        printf(b"\n%5d%12d\0" as *const u8 as *const std::os::raw::c_char, i, fusc(i));
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

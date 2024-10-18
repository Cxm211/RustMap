#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn is_pangram(mut s: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut alpha: *const std::os::raw::c_char = b"abcdefghjiklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0"
        as *const u8 as *const std::os::raw::c_char;
    let mut ch: std::os::raw::c_char = 0;
    let mut wasused: [std::os::raw::c_char; 26] = [
        0 as std::os::raw::c_int as std::os::raw::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut total: std::os::raw::c_int = 0 as std::os::raw::c_int;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        ch = *fresh0;
        if !(ch as std::os::raw::c_int != '\0' as i32) {
            break;
        }
        let mut p: *const std::os::raw::c_char = core::ptr::null();
        let mut idx: std::os::raw::c_int = 0;
        p = strchr(alpha, ch as std::os::raw::c_int);
        if p.is_null() {
            continue;
        }
        idx = (p.offset_from(alpha) as std::os::raw::c_long % 26 as std::os::raw::c_int as std::os::raw::c_long)
            as std::os::raw::c_int;
        total += (wasused[idx as usize] == 0) as std::os::raw::c_int;
        wasused[idx as usize] = 1 as std::os::raw::c_int as std::os::raw::c_char;
        if total == 26 as std::os::raw::c_int {
            return 1 as std::os::raw::c_int;
        }
    }
    return 0 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut tests: [*const std::os::raw::c_char; 2] = [
        b"The quick brown fox jumps over the lazy dog.\0" as *const u8
            as *const std::os::raw::c_char,
        b"The qu1ck brown fox jumps over the lazy d0g.\0" as *const u8
            as *const std::os::raw::c_char,
    ];
    i = 0 as std::os::raw::c_int;
    while i < 2 as std::os::raw::c_int {
        printf(
            b"\"%s\" is %sa pangram\n\0" as *const u8 as *const std::os::raw::c_char,
            tests[i as usize],
            if is_pangram(tests[i as usize]) != 0 {
                b"\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"not \0" as *const u8 as *const std::os::raw::c_char
            },
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn palindrome(mut s: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut l: std::os::raw::c_int = 0;
    l = strlen(s) as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < l / 2 as std::os::raw::c_int {
        if *s.offset(i as isize) as std::os::raw::c_int
            != *s.offset((l - i - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn palindrome_r(
    mut s: *const std::os::raw::c_char,
    mut b: std::os::raw::c_int,
    mut e: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if e - 1 as std::os::raw::c_int <= b {
        return 1 as std::os::raw::c_int;
    }
    if *s.offset(b as isize) as std::os::raw::c_int
        != *s.offset((e - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    return palindrome_r(s, b + 1 as std::os::raw::c_int, e - 1 as std::os::raw::c_int);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut t: *const std::os::raw::c_char = b"ingirumimusnocteetconsumimurigni\0" as *const u8
        as *const std::os::raw::c_char;
    let mut template: *const std::os::raw::c_char = b"sequence \"%s\" is%s palindrome\n\0"
        as *const u8 as *const std::os::raw::c_char;
    let mut l: std::os::raw::c_int = strlen(t) as std::os::raw::c_int;
    printf(
        template,
        t,
        if palindrome(t) != 0 {
            b"\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"n't\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    printf(
        template,
        t,
        if palindrome_r(t, 0 as std::os::raw::c_int, l) != 0 {
            b"\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"n't\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

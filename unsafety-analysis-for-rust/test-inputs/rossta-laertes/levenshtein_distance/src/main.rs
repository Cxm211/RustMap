#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn levenshtein(
    mut s: *const std::os::raw::c_char,
    mut ls: std::os::raw::c_int,
    mut t: *const std::os::raw::c_char,
    mut lt: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    if ls == 0 {
        return lt;
    }
    if lt == 0 {
        return ls;
    }
    if *s.offset((ls - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
        == *t.offset((lt - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
    {
        return levenshtein(s, ls - 1 as std::os::raw::c_int, t, lt - 1 as std::os::raw::c_int);
    }
    a = levenshtein(s, ls - 1 as std::os::raw::c_int, t, lt - 1 as std::os::raw::c_int);
    b = levenshtein(s, ls, t, lt - 1 as std::os::raw::c_int);
    c = levenshtein(s, ls - 1 as std::os::raw::c_int, t, lt);
    if a > b {
        a = b;
    }
    if a > c {
        a = c;
    }
    return a + 1 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut s1: *const std::os::raw::c_char = b"rosettacode\0" as *const u8
        as *const std::os::raw::c_char;
    let mut s2: *const std::os::raw::c_char = b"raisethysword\0" as *const u8
        as *const std::os::raw::c_char;
    printf(
        b"distance between `%s' and `%s': %d\n\0" as *const u8 as *const std::os::raw::c_char,
        s1,
        s2,
        levenshtein(s1, strlen(s1) as std::os::raw::c_int, s2, strlen(s2) as std::os::raw::c_int),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

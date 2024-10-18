#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
#[no_mangle]
pub unsafe extern "C" fn incr(mut s: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let mut i: std::os::raw::c_int = 0;
    let mut begin: std::os::raw::c_int = 0;
    let mut tail: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut neg: std::os::raw::c_int = (*s as std::os::raw::c_int == '-' as i32) as std::os::raw::c_int;
    let mut tgt: std::os::raw::c_char = (if neg != 0 { '0' as i32 } else { '9' as i32 })
        as std::os::raw::c_char;
    if strcmp(s, b"-1\0" as *const u8 as *const std::os::raw::c_char) == 0 {
        *s.offset(0 as std::os::raw::c_int as isize) = '0' as i32 as std::os::raw::c_char;
        *s.offset(1 as std::os::raw::c_int as isize) = '\0' as i32 as std::os::raw::c_char;
        return s;
    }
    len = strlen(s) as std::os::raw::c_int;
    begin = if *s as std::os::raw::c_int == '-' as i32 || *s as std::os::raw::c_int == '+' as i32 {
        1 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    };
    tail = len - 1 as std::os::raw::c_int;
    while tail >= begin && *s.offset(tail as isize) as std::os::raw::c_int == tgt as std::os::raw::c_int
    {
        tail -= 1;
        tail;
    }
    if tail < begin && neg == 0 {
        if begin == 0 {
            s = realloc(
                s as *mut u8,
                (len + 2 as std::os::raw::c_int) as std::os::raw::c_ulong,
            ) as *mut std::os::raw::c_char;
        }
        *s.offset(0 as std::os::raw::c_int as isize) = '1' as i32 as std::os::raw::c_char;
        i = 1 as std::os::raw::c_int;
        while i <= len - begin {
            *s.offset(i as isize) = '0' as i32 as std::os::raw::c_char;
            i += 1;
            i;
        }
        *s.offset((len + 1 as std::os::raw::c_int) as isize) = '\0' as i32 as std::os::raw::c_char;
    } else if tail == begin && neg != 0
        && *s.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == '1' as i32
    {
        i = 1 as std::os::raw::c_int;
        while i < len - begin {
            *s.offset(i as isize) = '9' as i32 as std::os::raw::c_char;
            i += 1;
            i;
        }
        *s.offset((len - 1 as std::os::raw::c_int) as isize) = '\0' as i32 as std::os::raw::c_char;
    } else {
        i = len - 1 as std::os::raw::c_int;
        while i > tail {
            *s
                .offset(
                    i as isize,
                ) = (if neg != 0 { '9' as i32 } else { '0' as i32 }) as std::os::raw::c_char;
            i -= 1;
            i;
        }
        let ref mut fresh0 = *s.offset(tail as isize);
        *fresh0 = (*fresh0 as std::os::raw::c_int
            + if neg != 0 { -(1 as std::os::raw::c_int) } else { 1 as std::os::raw::c_int })
            as std::os::raw::c_char;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn string_test(mut s: *const std::os::raw::c_char) {
    let mut ret: *mut std::os::raw::c_char = malloc(strlen(s)) as *mut std::os::raw::c_char;
    strcpy(ret, s);
    printf(b"text: %s\n\0" as *const u8 as *const std::os::raw::c_char, ret);
    ret = incr(ret);
    printf(b"  ->: %s\n\0" as *const u8 as *const std::os::raw::c_char, ret);
    free(ret as *mut u8);
}
unsafe fn main_0() -> std::os::raw::c_int {
    string_test(b"+0\0" as *const u8 as *const std::os::raw::c_char);
    string_test(b"-1\0" as *const u8 as *const std::os::raw::c_char);
    string_test(b"-41\0" as *const u8 as *const std::os::raw::c_char);
    string_test(b"+41\0" as *const u8 as *const std::os::raw::c_char);
    string_test(b"999\0" as *const u8 as *const std::os::raw::c_char);
    string_test(b"+999\0" as *const u8 as *const std::os::raw::c_char);
    string_test(
        b"109999999999999999999999999999999999999999\0" as *const u8
            as *const std::os::raw::c_char,
    );
    string_test(
        b"-100000000000000000000000000000000000000000000\0" as *const u8
            as *const std::os::raw::c_char,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

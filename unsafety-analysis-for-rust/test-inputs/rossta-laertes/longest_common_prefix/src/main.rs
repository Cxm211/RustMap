#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
// //use ::c2rust_out::*;
extern "C" {
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strncpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type __builtin_va_list = [crate::__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: * mut u8,
    pub reg_save_area: * mut u8,
}
impl std::default::Default for __va_list_tag {
    fn default() -> Self {
        __va_list_tag {
        gp_offset: std::os::raw::c_uint::default(),
        fp_offset: std::os::raw::c_uint::default(),
        overflow_arg_area: 0 as * mut u8,
        reg_save_area: 0 as * mut u8
        }
    }
}

pub type va_list = [crate::__va_list_tag; 1];
#[no_mangle]
pub unsafe extern "C" fn lcp(mut num: std::os::raw::c_int, mut args: ...) -> *mut std::os::raw::c_char {
    let mut vaList: std::ffi::VaListImpl;
    let mut vaList2: std::ffi::VaListImpl;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut min: std::os::raw::c_int = 0;
    let mut dest: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut strings: *mut *mut std::os::raw::c_char = malloc(
        (num as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_char;
    vaList = args.clone();
    vaList2 = args.clone();
    i = 0 as std::os::raw::c_int;
    while i < num {
        len = strlen(vaList.arg::<*mut std::os::raw::c_char>()) as std::os::raw::c_int;
        let ref mut fresh0 = *strings.offset(i as isize);
        *fresh0 = malloc(
            ((len + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_char;
        strcpy(*strings.offset(i as isize), vaList2.arg::<*mut std::os::raw::c_char>());
        if i == 0 as std::os::raw::c_int {
            min = len;
        } else if len < min {
            min = len;
        }
        i += 1;
        i;
    }
    if min == 0 as std::os::raw::c_int {
        return b"\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    }
    i = 0 as std::os::raw::c_int;
    while i < min {
        j = 1 as std::os::raw::c_int;
        while j < num {
            if *(*strings.offset(j as isize)).offset(i as isize) as std::os::raw::c_int
                != *(*strings.offset(0 as std::os::raw::c_int as isize)).offset(i as isize)
                    as std::os::raw::c_int
            {
                if i == 0 as std::os::raw::c_int {
                    return b"\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char
                } else {
                    dest = malloc(
                        (i as std::os::raw::c_ulong)
                            .wrapping_mul(
                                std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong,
                            ),
                    ) as *mut std::os::raw::c_char;
                    strncpy(
                        dest,
                        *strings.offset(0 as std::os::raw::c_int as isize),
                        (i - 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
                    );
                    return dest;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dest = malloc(
        ((min + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    strncpy(dest, *strings.offset(0 as std::os::raw::c_int as isize), min as std::os::raw::c_ulong);
    return dest;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(
            3 as std::os::raw::c_int,
            b"interspecies\0" as *const u8 as *const std::os::raw::c_char,
            b"interstellar\0" as *const u8 as *const std::os::raw::c_char,
            b"interstate\0" as *const u8 as *const std::os::raw::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(
            2 as std::os::raw::c_int,
            b"throne\0" as *const u8 as *const std::os::raw::c_char,
            b"throne\0" as *const u8 as *const std::os::raw::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(
            2 as std::os::raw::c_int,
            b"throne\0" as *const u8 as *const std::os::raw::c_char,
            b"dungeon\0" as *const u8 as *const std::os::raw::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(
            3 as std::os::raw::c_int,
            b"throne\0" as *const u8 as *const std::os::raw::c_char,
            b"\0" as *const u8 as *const std::os::raw::c_char,
            b"throne\0" as *const u8 as *const std::os::raw::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(1 as std::os::raw::c_int, b"cheese\0" as *const u8 as *const std::os::raw::c_char),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(1 as std::os::raw::c_int, b"\0" as *const u8 as *const std::os::raw::c_char),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(0 as std::os::raw::c_int, core::ptr::null_mut()),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(
            2 as std::os::raw::c_int,
            b"prefix\0" as *const u8 as *const std::os::raw::c_char,
            b"suffix\0" as *const u8 as *const std::os::raw::c_char,
        ),
    );
    printf(
        b"\nLongest common prefix : %s\0" as *const u8 as *const std::os::raw::c_char,
        lcp(
            2 as std::os::raw::c_int,
            b"foo\0" as *const u8 as *const std::os::raw::c_char,
            b"foobar\0" as *const u8 as *const std::os::raw::c_char,
        ),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = 0;
    let mut a2: std::os::raw::c_int = 0;
    let mut s: std::os::raw::c_int = 3 as std::os::raw::c_int;
    let mut s1: std::os::raw::c_int = 0;
    let mut s2: std::os::raw::c_int = 0;
    let mut r: [std::os::raw::c_int; 2201] = [0; 2201];
    memset(
        r.as_mut_ptr() as *mut u8,
        0 as std::os::raw::c_int,
        std::mem::size_of::<[std::os::raw::c_int; 2201]>() as std::os::raw::c_ulong,
    );
    let mut ab: *mut std::os::raw::c_int = calloc(
        (2200 as std::os::raw::c_int * 2200 as std::os::raw::c_int * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int)
            as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    a = 1 as std::os::raw::c_int;
    while a <= 2200 as std::os::raw::c_int {
        a2 = a * a;
        b = a;
        while b <= 2200 as std::os::raw::c_int {
            *ab.offset((a2 + b * b) as isize) = 1 as std::os::raw::c_int;
            b += 1;
            b;
        }
        a += 1;
        a;
    }
    c = 1 as std::os::raw::c_int;
    while c <= 2200 as std::os::raw::c_int {
        s1 = s;
        s += 2 as std::os::raw::c_int;
        s2 = s;
        d = c + 1 as std::os::raw::c_int;
        while d <= 2200 as std::os::raw::c_int {
            if *ab.offset(s1 as isize) != 0 {
                r[d as usize] = 1 as std::os::raw::c_int;
            }
            s1 += s2;
            s2 += 2 as std::os::raw::c_int;
            d += 1;
            d;
        }
        c += 1;
        c;
    }
    d = 1 as std::os::raw::c_int;
    while d <= 2200 as std::os::raw::c_int {
        if r[d as usize] == 0 {
            printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, d);
        }
        d += 1;
        d;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    free(ab as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

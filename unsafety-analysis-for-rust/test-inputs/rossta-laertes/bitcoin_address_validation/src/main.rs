#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn memcmp(
        _: *const u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    fn SHA256(
        d: *const std::os::raw::c_uchar,
        n: size_t,
        md: *mut std::os::raw::c_uchar,
    ) -> *mut std::os::raw::c_uchar;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub static mut coin_err: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
#[no_mangle]
pub unsafe extern "C" fn unbase58(
    mut s: *const std::os::raw::c_char,
    mut out: *mut std::os::raw::c_uchar,
) -> std::os::raw::c_int {
    static mut tmpl: *const std::os::raw::c_char = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz\0"
        as *const u8 as *const std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut p: *const std::os::raw::c_char = core::ptr::null();
    memset(
        out as *mut u8,
        0 as std::os::raw::c_int,
        25 as std::os::raw::c_int as std::os::raw::c_ulong,
    );
    i = 0 as std::os::raw::c_int;
    while *s.offset(i as isize) != 0 {
        p = strchr(tmpl, *s.offset(i as isize) as std::os::raw::c_int);
        if p.is_null() {
            coin_err = b"bad char\0" as *const u8 as *const std::os::raw::c_char;
            return 0 as std::os::raw::c_int;
        }
        c = p.offset_from(tmpl) as std::os::raw::c_long as std::os::raw::c_int;
        j = 25 as std::os::raw::c_int;
        loop {
            let fresh0 = j;
            j = j - 1;
            if !(fresh0 != 0) {
                break;
            }
            c += 58 as std::os::raw::c_int * *out.offset(j as isize) as std::os::raw::c_int;
            *out.offset(j as isize) = (c % 256 as std::os::raw::c_int) as std::os::raw::c_uchar;
            c /= 256 as std::os::raw::c_int;
        }
        if c != 0 {
            coin_err = b"address too long\0" as *const u8 as *const std::os::raw::c_char;
            return 0 as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn valid(mut s: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut dec: [std::os::raw::c_uchar; 32] = [0; 32];
    let mut d1: [std::os::raw::c_uchar; 32] = [0; 32];
    let mut d2: [std::os::raw::c_uchar; 32] = [0; 32];
    coin_err = b"\0" as *const u8 as *const std::os::raw::c_char;
    if unbase58(s, dec.as_mut_ptr()) == 0 {
        return 0 as std::os::raw::c_int;
    }
    SHA256(
        SHA256(dec.as_mut_ptr(), 21 as std::os::raw::c_int as size_t, d1.as_mut_ptr()),
        32 as std::os::raw::c_int as size_t,
        d2.as_mut_ptr(),
    );
    if memcmp(
        dec.as_mut_ptr().offset(21 as std::os::raw::c_int as isize) as *const u8,
        d2.as_mut_ptr() as *const u8,
        4 as std::os::raw::c_int as std::os::raw::c_ulong,
    ) != 0
    {
        coin_err = b"bad digest\0" as *const u8 as *const std::os::raw::c_char;
        return 0 as std::os::raw::c_int;
    }
    return 1 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut s: [*const std::os::raw::c_char; 5] = [
        b"1Q1pE5vPGEEMqRcVRMbtBK842Y6Pzo6nK9\0" as *const u8 as *const std::os::raw::c_char,
        b"1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62i\0" as *const u8 as *const std::os::raw::c_char,
        b"1Q1pE5vPGEEMqRcVRMbtBK842Y6Pzo6nJ9\0" as *const u8 as *const std::os::raw::c_char,
        b"1AGNa15ZQXAZUgFiqJ2i7Z2DPU2J6hW62I\0" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
    ];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while !(s[i as usize]).is_null() {
        let mut status: std::os::raw::c_int = valid(s[i as usize]);
        printf(
            b"%s: %s\n\0" as *const u8 as *const std::os::raw::c_char,
            s[i as usize],
            if status != 0 {
                b"Ok\0" as *const u8 as *const std::os::raw::c_char
            } else {
                coin_err
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

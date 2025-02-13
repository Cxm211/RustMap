#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gray_encode(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    return n ^ n >> 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gray_decode(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut p: std::os::raw::c_int = n;
    loop {
        n >>= 1 as std::os::raw::c_int;
        if !(n != 0) {
            break;
        }
        p ^= n;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn fmtbool(mut n: std::os::raw::c_int, mut buf: *mut std::os::raw::c_char) {
    let mut b: *mut std::os::raw::c_char = buf.offset(5 as std::os::raw::c_int as isize);
    *b = 0 as std::os::raw::c_int as std::os::raw::c_char;
    loop {
        b = b.offset(-1);
        *b = ('0' as i32 + (n & 1 as std::os::raw::c_int)) as std::os::raw::c_char;
        n >>= 1 as std::os::raw::c_int;
        if !(b != buf) {
            break;
        }
    };
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut g: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut bi: [std::os::raw::c_char; 6] = [0; 6];
    let mut bg: [std::os::raw::c_char; 6] = [0; 6];
    let mut bb: [std::os::raw::c_char; 6] = [0; 6];
    i = 0 as std::os::raw::c_int;
    while i < 32 as std::os::raw::c_int {
        g = gray_encode(i);
        b = gray_decode(g);
        fmtbool(i, bi.as_mut_ptr());
        fmtbool(g, bg.as_mut_ptr());
        fmtbool(b, bb.as_mut_ptr());
        printf(
            b"%2d : %5s => %5s => %5s : %2d\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            bi.as_mut_ptr(),
            bg.as_mut_ptr(),
            bb.as_mut_ptr(),
            b,
        );
        i += 1;
        i;
    }
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
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

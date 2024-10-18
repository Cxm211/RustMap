#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn exp(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut e: std::os::raw::c_double = 0.;
    puts(
        b"The double precision in C give about 15 significant digits.\nValues below are presented with 16 digits after the decimal point.\n\0"
            as *const u8 as *const std::os::raw::c_char,
    );
    e = exp(1 as std::os::raw::c_int as std::os::raw::c_double);
    printf(b"Euler constant e = %.16lf\n\0" as *const u8 as *const std::os::raw::c_char, e);
    let mut n: std::os::raw::c_int = 8192 as std::os::raw::c_int;
    e = 1.0f64 + 1.0f64 / n as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < 13 as std::os::raw::c_int {
        e *= e;
        i += 1;
        i;
    }
    printf(b"Euler constant e = %.16lf\n\0" as *const u8 as *const std::os::raw::c_char, e);
    let N: std::os::raw::c_int = 1000 as std::os::raw::c_int;
    let mut a: [std::os::raw::c_double; 1000] = [0.; 1000];
    a[0 as std::os::raw::c_int as usize] = 1.0f64;
    let mut i_0: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while i_0 < N {
        a[i_0 as usize] = a[(i_0 - 1 as std::os::raw::c_int) as usize] / i_0 as std::os::raw::c_double;
        i_0 += 1;
        i_0;
    }
    e = 1.0f64;
    let mut i_1: std::os::raw::c_int = N - 1 as std::os::raw::c_int;
    while i_1 > 0 as std::os::raw::c_int {
        e += a[i_1 as usize];
        i_1 -= 1;
        i_1;
    }
    printf(b"Euler constant e = %.16lf\n\0" as *const u8 as *const std::os::raw::c_char, e);
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

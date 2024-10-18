#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rms(
    mut v: *mut std::os::raw::c_double,
    mut n: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut i: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_double = 0.0f64;
    i = 0 as std::os::raw::c_int;
    while i < n {
        sum += *v.offset(i as isize) * *v.offset(i as isize);
        i += 1;
        i;
    }
    return sqrt(sum / n as std::os::raw::c_double);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut v: [std::os::raw::c_double; 10] = [
        1.0f64,
        2.0f64,
        3.0f64,
        4.0f64,
        5.0f64,
        6.0f64,
        7.0f64,
        8.0f64,
        9.0f64,
        10.0f64,
    ];
    printf(
        b"%f\n\0" as *const u8 as *const std::os::raw::c_char,
        rms(
            v.as_mut_ptr(),
            (std::mem::size_of::<[std::os::raw::c_double; 10]>() as std::os::raw::c_ulong)
                .wrapping_div(std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong)
                as std::os::raw::c_int,
        ),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

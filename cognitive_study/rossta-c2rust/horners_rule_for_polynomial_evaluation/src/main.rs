#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn horner(
    mut coeffs: *mut std::os::raw::c_double,
    mut s: std::os::raw::c_int,
    mut x: std::os::raw::c_double,
) -> std::os::raw::c_double {
    let mut i: std::os::raw::c_int = 0;
    let mut res: std::os::raw::c_double = 0.0f64;
    i = s - 1 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        res = res * x + *coeffs.offset(i as isize);
        i -= 1;
        i;
    }
    return res;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut coeffs: [std::os::raw::c_double; 4] = [-19.0f64, 7.0f64, -4.0f64, 6.0f64];
    printf(
        b"%5.1f\n\0" as *const u8 as *const std::os::raw::c_char,
        horner(
            coeffs.as_mut_ptr(),
            (::core::mem::size_of::<[std::os::raw::c_double; 4]>() as std::os::raw::c_ulong)
                .wrapping_div(::core::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong)
                as std::os::raw::c_int,
            3.0f64,
        ),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

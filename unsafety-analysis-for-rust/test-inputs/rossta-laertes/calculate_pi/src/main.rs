#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
#[no_mangle]
pub unsafe extern "C" fn agm(
    in1: std::os::raw::c_int,
    in2: std::os::raw::c_int,
    mut out1: std::os::raw::c_int,
    mut out2: std::os::raw::c_int,
) {}
unsafe fn main_0() -> std::os::raw::c_int {
    mpf_set_default_prec(300000 as std::os::raw::c_int);
    let mut n: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        n += n;
        n += n;
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapRange(
    mut a1: std::os::raw::c_double,
    mut a2: std::os::raw::c_double,
    mut b1: std::os::raw::c_double,
    mut b2: std::os::raw::c_double,
    mut s: std::os::raw::c_double,
) -> std::os::raw::c_double {
    return b1 + (s - a1) * (b2 - b1) / (a2 - a1);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    puts(
        b"Mapping [0,10] to [-1,0] at intervals of 1:\0" as *const u8
            as *const std::os::raw::c_char,
    );
    i = 0 as std::os::raw::c_int;
    while i <= 10 as std::os::raw::c_int {
        printf(
            b"f(%d) = %g\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            mapRange(
                0 as std::os::raw::c_int as std::os::raw::c_double,
                10 as std::os::raw::c_int as std::os::raw::c_double,
                -(1 as std::os::raw::c_int) as std::os::raw::c_double,
                0 as std::os::raw::c_int as std::os::raw::c_double,
                i as std::os::raw::c_double,
            ),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

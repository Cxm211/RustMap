#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut r: std::os::raw::c_float = 7.125f64 as std::os::raw::c_float;
    printf(b" %9.3f\n\0" as *const u8 as *const std::os::raw::c_char, -r as std::os::raw::c_double);
    printf(b" %9.3f\n\0" as *const u8 as *const std::os::raw::c_char, r as std::os::raw::c_double);
    printf(b" %-9.3f\n\0" as *const u8 as *const std::os::raw::c_char, r as std::os::raw::c_double);
    printf(b" %09.3f\n\0" as *const u8 as *const std::os::raw::c_char, -r as std::os::raw::c_double);
    printf(b" %09.3f\n\0" as *const u8 as *const std::os::raw::c_char, r as std::os::raw::c_double);
    printf(b" %-09.3f\n\0" as *const u8 as *const std::os::raw::c_char, r as std::os::raw::c_double);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

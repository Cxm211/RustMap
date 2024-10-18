#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn scanf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    scanf(
        b"%d%d\0" as *const u8 as *const std::os::raw::c_char,
        &mut a as *mut std::os::raw::c_int,
        &mut b as *mut std::os::raw::c_int,
    );
    printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, a + b);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

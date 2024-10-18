#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub static mut val: std::os::raw::c_int = 2 as std::os::raw::c_int * 3 as std::os::raw::c_int * 4 as std::os::raw::c_int
    * 5 as std::os::raw::c_int * 6 as std::os::raw::c_int * 7 as std::os::raw::c_int * 8 as std::os::raw::c_int
    * 9 as std::os::raw::c_int * 10 as std::os::raw::c_int;
unsafe fn main_0() -> std::os::raw::c_int {
    printf(b"10! = %d\n\0" as *const u8 as *const std::os::raw::c_char, val);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

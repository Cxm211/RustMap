#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut object: *mut std::os::raw::c_char = core::ptr::null_mut();
    if object.is_null() {
        puts(b"object is null\0" as *const u8 as *const std::os::raw::c_char);
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut fruit: [*const std::os::raw::c_char; 2] = [
        b"apples\0" as *const u8 as *const std::os::raw::c_char,
        b"oranges\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut length: std::os::raw::c_int = (std::mem::size_of::<[*const std::os::raw::c_char; 2]>()
        as std::os::raw::c_ulong)
        .wrapping_div(std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, length);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

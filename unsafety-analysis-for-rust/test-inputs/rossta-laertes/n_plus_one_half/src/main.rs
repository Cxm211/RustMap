#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i <= 10 as std::os::raw::c_int {
        printf(b"%d\0" as *const u8 as *const std::os::raw::c_char, i);
        printf(
            if i == 10 as std::os::raw::c_int {
                b"\n\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b", \0" as *const u8 as *const std::os::raw::c_char
            },
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

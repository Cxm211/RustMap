#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    printf(
        b"<table style=\"text-align:center; border: 1px solid\"><th></th><th>X</th><th>Y</th><th>Z</th>\0"
            as *const u8 as *const std::os::raw::c_char,
    );
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        printf(
            b"<tr><th>%d</th><td>%d</td><td>%d</td><td>%d</td></tr>\0" as *const u8
                as *const std::os::raw::c_char,
            i,
            rand() % 10000 as std::os::raw::c_int,
            rand() % 10000 as std::os::raw::c_int,
            rand() % 10000 as std::os::raw::c_int,
        );
        i += 1;
        i;
    }
    printf(b"</table>\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

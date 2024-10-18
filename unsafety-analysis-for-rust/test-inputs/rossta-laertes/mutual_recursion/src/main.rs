#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn F(n: std::os::raw::c_int) -> std::os::raw::c_int {
    return if n == 0 as std::os::raw::c_int {
        1 as std::os::raw::c_int
    } else {
        n - M(F(n - 1 as std::os::raw::c_int))
    };
}
#[no_mangle]
pub extern "C" fn M(n: std::os::raw::c_int) -> std::os::raw::c_int {
    return if n == 0 as std::os::raw::c_int {
        0 as std::os::raw::c_int
    } else {
        n - F(M(n - 1 as std::os::raw::c_int))
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 20 as std::os::raw::c_int {
        printf(b"%2d \0" as *const u8 as *const std::os::raw::c_char, F(i));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < 20 as std::os::raw::c_int {
        printf(b"%2d \0" as *const u8 as *const std::os::raw::c_char, M(i));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn a(mut in_0: bool) -> bool {
    printf(b"I am a\n\0" as *const u8 as *const std::os::raw::c_char);
    return in_0;
}
#[no_mangle]
pub unsafe extern "C" fn b(mut in_0: bool) -> bool {
    printf(b"I am b\n\0" as *const u8 as *const std::os::raw::c_char);
    return in_0;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: bool = false;
    x = a(0 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0
        && b(1 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0;
    printf(
        b"false && true = %s\n\n\0" as *const u8 as *const std::os::raw::c_char,
        if x as std::os::raw::c_int != 0 {
            b"true\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"false\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    x = a(1 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0
        || b(0 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0;
    printf(
        b"true || false = %s\n\n\0" as *const u8 as *const std::os::raw::c_char,
        if x as std::os::raw::c_int != 0 {
            b"true\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"false\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    x = a(1 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0
        && b(0 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0;
    printf(
        b"true && false = %s\n\n\0" as *const u8 as *const std::os::raw::c_char,
        if x as std::os::raw::c_int != 0 {
            b"true\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"false\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    x = a(0 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0
        || b(0 as std::os::raw::c_int != 0) as std::os::raw::c_int != 0;
    printf(
        b"false || false = %s\n\n\0" as *const u8 as *const std::os::raw::c_char,
        if x as std::os::raw::c_int != 0 {
            b"true\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"false\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

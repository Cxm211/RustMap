#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F(n: libc::c_int) -> libc::c_int {
    return if n == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        n - M(F(n - 1 as libc::c_int))
    };
}
#[no_mangle]
pub unsafe extern "C" fn M(n: libc::c_int) -> libc::c_int {
    return if n == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        n - F(M(n - 1 as libc::c_int))
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        printf(b"%2d \0" as *const u8 as *const libc::c_char, F(i));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        printf(b"%2d \0" as *const u8 as *const libc::c_char, M(i));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

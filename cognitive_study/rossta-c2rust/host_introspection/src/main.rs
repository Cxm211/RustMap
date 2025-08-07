#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut one: std::os::raw::c_int = 1 as std::os::raw::c_int;
    printf(
        b"word size = %d bits\n\0" as *const u8 as *const std::os::raw::c_char,
        (8 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
    );
    if *(&mut one as *mut std::os::raw::c_int as *mut std::os::raw::c_char) != 0 {
        printf(b"little endian\n\0" as *const u8 as *const std::os::raw::c_char);
    } else {
        printf(b"big endian\n\0" as *const u8 as *const std::os::raw::c_char);
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

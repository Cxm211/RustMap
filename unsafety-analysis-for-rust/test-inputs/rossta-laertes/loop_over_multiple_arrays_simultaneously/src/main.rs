#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub static mut a1: [std::os::raw::c_char; 3] = [
    'a' as i32 as std::os::raw::c_char,
    'b' as i32 as std::os::raw::c_char,
    'c' as i32 as std::os::raw::c_char,
];
#[no_mangle]
pub static mut a2: [std::os::raw::c_char; 3] = [
    'A' as i32 as std::os::raw::c_char,
    'B' as i32 as std::os::raw::c_char,
    'C' as i32 as std::os::raw::c_char,
];
#[no_mangle]
pub static mut a3: [std::os::raw::c_int; 3] = [
    1 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    3 as std::os::raw::c_int,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < 3 as std::os::raw::c_int {
        printf(
            b"%c%c%i\n\0" as *const u8 as *const std::os::raw::c_char,
            a1[i as usize] as std::os::raw::c_int,
            a2[i as usize] as std::os::raw::c_int,
            a3[i as usize],
        );
        i += 1;
        i;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

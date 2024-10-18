#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn scanf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn leonardo(
    mut a: std::os::raw::c_int,
    mut b: std::os::raw::c_int,
    mut step: std::os::raw::c_int,
    mut num: std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut temp: std::os::raw::c_int = 0;
    printf(b"First 25 Leonardo numbers : \n\0" as *const u8 as *const std::os::raw::c_char);
    i = 1 as std::os::raw::c_int;
    while i <= num {
        if i == 1 as std::os::raw::c_int {
            printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, a);
        } else if i == 2 as std::os::raw::c_int {
            printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, b);
        } else {
            printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, a + b + step);
            temp = a;
            a = b;
            b = temp + b + step;
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut step: std::os::raw::c_int = 0;
    printf(
        b"Enter first two Leonardo numbers and increment step : \0" as *const u8
            as *const std::os::raw::c_char,
    );
    scanf(
        b"%d%d%d\0" as *const u8 as *const std::os::raw::c_char,
        &mut a as *mut std::os::raw::c_int,
        &mut b as *mut std::os::raw::c_int,
        &mut step as *mut std::os::raw::c_int,
    );
    leonardo(a, b, step, 25 as std::os::raw::c_int);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

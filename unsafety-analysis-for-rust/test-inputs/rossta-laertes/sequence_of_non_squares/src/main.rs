#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn floor(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn nonsqr(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    return n + (0.5f64 + sqrt(n as std::os::raw::c_double)) as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < 23 as std::os::raw::c_int {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, nonsqr(i));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 1 as std::os::raw::c_int;
    while i < 1000000 as std::os::raw::c_int {
        let mut j: std::os::raw::c_double = sqrt(nonsqr(i) as std::os::raw::c_double);
        if j != floor(j) {} else {
            __assert_fail(
                b"j != floor(j)\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                21 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 11],
                    &[std::os::raw::c_char; 11],
                >(b"int main()\0"))
                    .as_ptr(),
            );
        }
        'c_1861: {
            if j != floor(j) {} else {
                __assert_fail(
                    b"j != floor(j)\0" as *const u8 as *const std::os::raw::c_char,
                    b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                    21 as std::os::raw::c_int as std::os::raw::c_uint,
                    (*std::mem::transmute::<
                        &[u8; 11],
                        &[std::os::raw::c_char; 11],
                    >(b"int main()\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

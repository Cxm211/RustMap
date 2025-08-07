#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut inf: std::os::raw::c_double = 1 as std::os::raw::c_int as std::os::raw::c_double / 0.0f64;
    let mut minus_inf: std::os::raw::c_double = -(1 as std::os::raw::c_int) as std::os::raw::c_double / 0.0f64;
    let mut minus_zero: std::os::raw::c_double = -(1 as std::os::raw::c_int) as std::os::raw::c_double / inf;
    let mut nan: std::os::raw::c_double = 0.0f64 / 0.0f64;
    printf(b"positive infinity: %f\n\0" as *const u8 as *const std::os::raw::c_char, inf);
    printf(b"negative infinity: %f\n\0" as *const u8 as *const std::os::raw::c_char, minus_inf);
    printf(b"negative zero: %f\n\0" as *const u8 as *const std::os::raw::c_char, minus_zero);
    printf(b"not a number: %f\n\0" as *const u8 as *const std::os::raw::c_char, nan);
    printf(b"+inf + 2.0 = %f\n\0" as *const u8 as *const std::os::raw::c_char, inf + 2.0f64);
    printf(b"+inf - 10.1 = %f\n\0" as *const u8 as *const std::os::raw::c_char, inf - 10.1f64);
    printf(b"+inf + -inf = %f\n\0" as *const u8 as *const std::os::raw::c_char, inf + minus_inf);
    printf(b"0.0 * +inf = %f\n\0" as *const u8 as *const std::os::raw::c_char, 0.0f64 * inf);
    printf(
        b"1.0/-0.0 = %f\n\0" as *const u8 as *const std::os::raw::c_char,
        1.0f64 / minus_zero,
    );
    printf(b"NaN + 1.0 = %f\n\0" as *const u8 as *const std::os::raw::c_char, nan + 1.0f64);
    printf(b"NaN + NaN = %f\n\0" as *const u8 as *const std::os::raw::c_char, nan + nan);
    printf(
        b"NaN == NaN = %s\n\0" as *const u8 as *const std::os::raw::c_char,
        if nan == nan {
            b"true\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"false\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    printf(
        b"0.0 == -0.0 = %s\n\0" as *const u8 as *const std::os::raw::c_char,
        if 0.0f64 == minus_zero {
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

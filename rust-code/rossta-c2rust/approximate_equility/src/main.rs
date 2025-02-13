#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn approxEquals(
    mut value: std::os::raw::c_double,
    mut other: std::os::raw::c_double,
    mut epsilon: std::os::raw::c_double,
) -> bool {
    return fabs(value - other) < epsilon;
}
#[no_mangle]
pub unsafe extern "C" fn test(mut a: std::os::raw::c_double, mut b: std::os::raw::c_double) {
    let mut epsilon: std::os::raw::c_double = 1e-18f64;
    printf(
        b"%f, %f => %d\n\0" as *const u8 as *const std::os::raw::c_char,
        a,
        b,
        approxEquals(a, b, epsilon) as std::os::raw::c_int,
    );
}
unsafe fn main_0() -> std::os::raw::c_int {
    test(100000000000000.01f64, 100000000000000.011f64);
    test(100.01f64, 100.011f64);
    test(10000000000000.001f64 / 10000.0f64, 1000000000.0000001000f64);
    test(0.001f64, 0.0010000001f64);
    test(0.000000000000000000000101f64, 0.0f64);
    test(sqrt(2.0f64) * sqrt(2.0f64), 2.0f64);
    test(-sqrt(2.0f64) * sqrt(2.0f64), -2.0f64);
    test(3.14159265358979323846f64, 3.14159265358979324f64);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

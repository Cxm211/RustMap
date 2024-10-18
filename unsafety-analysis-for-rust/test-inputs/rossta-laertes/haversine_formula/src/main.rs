#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn asin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn dist(
    mut th1: std::os::raw::c_double,
    mut ph1: std::os::raw::c_double,
    mut th2: std::os::raw::c_double,
    mut ph2: std::os::raw::c_double,
) -> std::os::raw::c_double {
    let mut dx: std::os::raw::c_double = 0.;
    let mut dy: std::os::raw::c_double = 0.;
    let mut dz: std::os::raw::c_double = 0.;
    ph1 -= ph2;
    ph1 *= 3.1415926536f64 / 180 as std::os::raw::c_int as std::os::raw::c_double;
    th1 *= 3.1415926536f64 / 180 as std::os::raw::c_int as std::os::raw::c_double;
    th2 *= 3.1415926536f64 / 180 as std::os::raw::c_int as std::os::raw::c_double;
    dz = sin(th1) - sin(th2);
    dx = cos(ph1) * cos(th1) - cos(th2);
    dy = sin(ph1) * cos(th1);
    return asin(sqrt(dx * dx + dy * dy + dz * dz) / 2 as std::os::raw::c_int as std::os::raw::c_double)
        * 2 as std::os::raw::c_int as std::os::raw::c_double * 6371 as std::os::raw::c_int as std::os::raw::c_double;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut d: std::os::raw::c_double = dist(36.12f64, -86.67f64, 33.94f64, -118.4f64);
    printf(
        b"dist: %.1f km (%.1f mi.)\n\0" as *const u8 as *const std::os::raw::c_char,
        d,
        d / 1.609344f64,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

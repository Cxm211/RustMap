#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn rand() -> std::os::raw::c_int;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn log(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn drand() -> std::os::raw::c_double {
    return (rand() as std::os::raw::c_double + 1.0f64)
        / (2147483647 as std::os::raw::c_int as std::os::raw::c_double + 1.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn random_normal() -> std::os::raw::c_double {
    return sqrt(-(2 as std::os::raw::c_int) as std::os::raw::c_double * log(drand()))
        * cos(2 as std::os::raw::c_int as std::os::raw::c_double * 3.14159265358979323846f64 * drand());
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut rands: [std::os::raw::c_double; 1000] = [0.; 1000];
    i = 0 as std::os::raw::c_int;
    while i < 1000 as std::os::raw::c_int {
        rands[i as usize] = 1.0f64 + 0.5f64 * random_normal();
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

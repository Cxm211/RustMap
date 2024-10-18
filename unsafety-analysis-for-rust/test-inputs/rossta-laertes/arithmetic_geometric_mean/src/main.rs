#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn scanf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn exit(_: std::os::raw::c_int) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn agm(
    mut a: std::os::raw::c_double,
    mut g: std::os::raw::c_double,
) -> std::os::raw::c_double {
    let mut iota: std::os::raw::c_double = 1.0E-16f64;
    let mut a1: std::os::raw::c_double = 0.;
    let mut g1: std::os::raw::c_double = 0.;
    if a * g < 0.0f64 {
        printf(
            b"arithmetic-geometric mean undefined when x*y<0\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
        exit(1 as std::os::raw::c_int);
    }
    while fabs(a - g) > iota {
        a1 = (a + g) / 2.0f64;
        g1 = sqrt(a * g);
        a = a1;
        g = g1;
    }
    return a;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: std::os::raw::c_double = 0.;
    let mut y: std::os::raw::c_double = 0.;
    printf(b"Enter two numbers: \0" as *const u8 as *const std::os::raw::c_char);
    scanf(
        b"%lf%lf\0" as *const u8 as *const std::os::raw::c_char,
        &mut x as *mut std::os::raw::c_double,
        &mut y as *mut std::os::raw::c_double,
    );
    printf(
        b"The arithmetic-geometric mean is %lf\n\0" as *const u8 as *const std::os::raw::c_char,
        agm(x, y),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn atan2(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meanAngle(
    mut angles: *mut std::os::raw::c_double,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut y_part: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut x_part: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < size {
        x_part
            += cos(
                *angles.offset(i as isize) * 3.14159265358979323846f64
                    / 180 as std::os::raw::c_int as std::os::raw::c_double,
            );
        y_part
            += sin(
                *angles.offset(i as isize) * 3.14159265358979323846f64
                    / 180 as std::os::raw::c_int as std::os::raw::c_double,
            );
        i += 1;
        i;
    }
    return atan2(y_part / size as std::os::raw::c_double, x_part / size as std::os::raw::c_double)
        * 180 as std::os::raw::c_int as std::os::raw::c_double / 3.14159265358979323846f64;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut angleSet1: [std::os::raw::c_double; 2] = [
        350 as std::os::raw::c_int as std::os::raw::c_double,
        10 as std::os::raw::c_int as std::os::raw::c_double,
    ];
    let mut angleSet2: [std::os::raw::c_double; 4] = [
        90 as std::os::raw::c_int as std::os::raw::c_double,
        180 as std::os::raw::c_int as std::os::raw::c_double,
        270 as std::os::raw::c_int as std::os::raw::c_double,
        360 as std::os::raw::c_int as std::os::raw::c_double,
    ];
    let mut angleSet3: [std::os::raw::c_double; 3] = [
        10 as std::os::raw::c_int as std::os::raw::c_double,
        20 as std::os::raw::c_int as std::os::raw::c_double,
        30 as std::os::raw::c_int as std::os::raw::c_double,
    ];
    printf(
        b"\nMean Angle for 1st set : %lf degrees\0" as *const u8 as *const std::os::raw::c_char,
        meanAngle(angleSet1.as_mut_ptr(), 2 as std::os::raw::c_int),
    );
    printf(
        b"\nMean Angle for 2nd set : %lf degrees\0" as *const u8 as *const std::os::raw::c_char,
        meanAngle(angleSet2.as_mut_ptr(), 4 as std::os::raw::c_int),
    );
    printf(
        b"\nMean Angle for 3rd set : %lf degrees\n\0" as *const u8
            as *const std::os::raw::c_char,
        meanAngle(angleSet3.as_mut_ptr(), 3 as std::os::raw::c_int),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

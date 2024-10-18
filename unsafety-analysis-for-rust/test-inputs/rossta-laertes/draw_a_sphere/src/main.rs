#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn ceil(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn floor(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub static mut shades: *const std::os::raw::c_char = b".:!*oe&#%@\0" as *const u8
    as *const std::os::raw::c_char;
#[no_mangle]
pub static mut light: [std::os::raw::c_double; 3] = [
    30 as std::os::raw::c_int as std::os::raw::c_double,
    30 as std::os::raw::c_int as std::os::raw::c_double,
    -(50 as std::os::raw::c_int) as std::os::raw::c_double,
];
#[no_mangle]
pub unsafe extern "C" fn normalize(mut v: *mut std::os::raw::c_double) {
    let mut len: std::os::raw::c_double = sqrt(
        *v.offset(0 as std::os::raw::c_int as isize) * *v.offset(0 as std::os::raw::c_int as isize)
            + *v.offset(1 as std::os::raw::c_int as isize) * *v.offset(1 as std::os::raw::c_int as isize)
            + *v.offset(2 as std::os::raw::c_int as isize) * *v.offset(2 as std::os::raw::c_int as isize),
    );
    *v.offset(0 as std::os::raw::c_int as isize) /= len;
    *v.offset(1 as std::os::raw::c_int as isize) /= len;
    *v.offset(2 as std::os::raw::c_int as isize) /= len;
}
#[no_mangle]
pub unsafe extern "C" fn dot(
    mut x: *mut std::os::raw::c_double,
    mut y: *mut std::os::raw::c_double,
) -> std::os::raw::c_double {
    let mut d: std::os::raw::c_double = *x.offset(0 as std::os::raw::c_int as isize)
        * *y.offset(0 as std::os::raw::c_int as isize)
        + *x.offset(1 as std::os::raw::c_int as isize) * *y.offset(1 as std::os::raw::c_int as isize)
        + *x.offset(2 as std::os::raw::c_int as isize) * *y.offset(2 as std::os::raw::c_int as isize);
    return if d < 0 as std::os::raw::c_int as std::os::raw::c_double {
        -d
    } else {
        0 as std::os::raw::c_int as std::os::raw::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn draw_sphere(
    mut R: std::os::raw::c_double,
    mut k: std::os::raw::c_double,
    mut ambient: std::os::raw::c_double,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut intensity: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_double = 0.;
    let mut vec: [std::os::raw::c_double; 3] = [0.; 3];
    let mut x: std::os::raw::c_double = 0.;
    let mut y: std::os::raw::c_double = 0.;
    i = floor(-R) as std::os::raw::c_int;
    while i as std::os::raw::c_double <= ceil(R) {
        x = i as std::os::raw::c_double + 0.5f64;
        j = floor(-(2 as std::os::raw::c_int) as std::os::raw::c_double * R) as std::os::raw::c_int;
        while j as std::os::raw::c_double <= ceil(2 as std::os::raw::c_int as std::os::raw::c_double * R) {
            y = j as std::os::raw::c_double / 2.0f64 + 0.5f64;
            if x * x + y * y <= R * R {
                vec[0 as std::os::raw::c_int as usize] = x;
                vec[1 as std::os::raw::c_int as usize] = y;
                vec[2 as std::os::raw::c_int as usize] = sqrt(R * R - x * x - y * y);
                normalize(vec.as_mut_ptr());
                b = pow(dot(light.as_mut_ptr(), vec.as_mut_ptr()), k) + ambient;
                intensity = ((1 as std::os::raw::c_int as std::os::raw::c_double - b)
                    * (std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        as std::os::raw::c_double) as std::os::raw::c_int;
                if intensity < 0 as std::os::raw::c_int {
                    intensity = 0 as std::os::raw::c_int;
                }
                if intensity as std::os::raw::c_ulong
                    >= (std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                {
                    intensity = (std::mem::size_of::<*const std::os::raw::c_char>()
                        as std::os::raw::c_ulong)
                        .wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
                }
                putchar(*shades.offset(intensity as isize) as std::os::raw::c_int);
            } else {
                putchar(' ' as i32);
            }
            j += 1;
            j;
        }
        putchar('\n' as i32);
        i += 1;
        i;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    normalize(light.as_mut_ptr());
    draw_sphere(
        20 as std::os::raw::c_int as std::os::raw::c_double,
        4 as std::os::raw::c_int as std::os::raw::c_double,
        0.1f64,
    );
    draw_sphere(
        10 as std::os::raw::c_int as std::os::raw::c_double,
        2 as std::os::raw::c_int as std::os::raw::c_double,
        0.4f64,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

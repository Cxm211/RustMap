#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rk4(
    mut f: Option::<
        unsafe extern "C" fn(std::os::raw::c_double, std::os::raw::c_double) -> std::os::raw::c_double,
    >,
    mut dx: std::os::raw::c_double,
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
) -> std::os::raw::c_double {
    let mut k1: std::os::raw::c_double = dx * f.expect("non-null function pointer")(x, y);
    let mut k2: std::os::raw::c_double = dx
        * f
            .expect(
                "non-null function pointer",
            )(
            x + dx / 2 as std::os::raw::c_int as std::os::raw::c_double,
            y + k1 / 2 as std::os::raw::c_int as std::os::raw::c_double,
        );
    let mut k3: std::os::raw::c_double = dx
        * f
            .expect(
                "non-null function pointer",
            )(
            x + dx / 2 as std::os::raw::c_int as std::os::raw::c_double,
            y + k2 / 2 as std::os::raw::c_int as std::os::raw::c_double,
        );
    let mut k4: std::os::raw::c_double = dx
        * f.expect("non-null function pointer")(x + dx, y + k3);
    return y
        + (k1 + 2 as std::os::raw::c_int as std::os::raw::c_double * k2
            + 2 as std::os::raw::c_int as std::os::raw::c_double * k3 + k4)
            / 6 as std::os::raw::c_int as std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rate(
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
) -> std::os::raw::c_double {
    return x * sqrt(y);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut y: *mut std::os::raw::c_double = core::ptr::null_mut();
    let mut x: std::os::raw::c_double = 0.;
    let mut y2: std::os::raw::c_double = 0.;
    let mut x0: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut x1: std::os::raw::c_double = 10 as std::os::raw::c_int as std::os::raw::c_double;
    let mut dx: std::os::raw::c_double = 0.1f64;
    let mut i: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = (1 as std::os::raw::c_int as std::os::raw::c_double + (x1 - x0) / dx)
        as std::os::raw::c_int;
    y = malloc(
        (std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong)
            .wrapping_mul(n as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_double;
    *y.offset(0 as std::os::raw::c_int as isize) = 1 as std::os::raw::c_int as std::os::raw::c_double;
    i = 1 as std::os::raw::c_int;
    while i < n {
        *y
            .offset(
                i as isize,
            ) = rk4(
            Some(
                rate
                    as unsafe extern "C" fn(
                        std::os::raw::c_double,
                        std::os::raw::c_double,
                    ) -> std::os::raw::c_double,
            ),
            dx,
            x0 + dx * (i - 1 as std::os::raw::c_int) as std::os::raw::c_double,
            *y.offset((i - 1 as std::os::raw::c_int) as isize),
        );
        i += 1;
        i;
    }
    printf(b"x\ty\trel. err.\n------------\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < n {
        x = x0 + dx * i as std::os::raw::c_double;
        y2 = pow(
            x * x / 4 as std::os::raw::c_int as std::os::raw::c_double
                + 1 as std::os::raw::c_int as std::os::raw::c_double,
            2 as std::os::raw::c_int as std::os::raw::c_double,
        );
        printf(
            b"%g\t%g\t%g\n\0" as *const u8 as *const std::os::raw::c_char,
            x,
            *y.offset(i as isize),
            *y.offset(i as isize) / y2 - 1 as std::os::raw::c_int as std::os::raw::c_double,
        );
        i += 10 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rk4(
    mut f: Option::<
        unsafe extern "C" fn(libc::c_double, libc::c_double) -> libc::c_double,
    >,
    mut dx: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    let mut k1: libc::c_double = dx * f.expect("non-null function pointer")(x, y);
    let mut k2: libc::c_double = dx
        * f
            .expect(
                "non-null function pointer",
            )(
            x + dx / 2 as libc::c_int as libc::c_double,
            y + k1 / 2 as libc::c_int as libc::c_double,
        );
    let mut k3: libc::c_double = dx
        * f
            .expect(
                "non-null function pointer",
            )(
            x + dx / 2 as libc::c_int as libc::c_double,
            y + k2 / 2 as libc::c_int as libc::c_double,
        );
    let mut k4: libc::c_double = dx
        * f.expect("non-null function pointer")(x + dx, y + k3);
    return y
        + (k1 + 2 as libc::c_int as libc::c_double * k2
            + 2 as libc::c_int as libc::c_double * k3 + k4)
            / 6 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rate(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    return x * sqrt(y);
}
unsafe fn main_0() -> libc::c_int {
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut x0: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x1: libc::c_double = 10 as libc::c_int as libc::c_double;
    let mut dx: libc::c_double = 0.1f64;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (1 as libc::c_int as libc::c_double + (x1 - x0) / dx)
        as libc::c_int;
    y = malloc(
        (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_double;
    *y.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    i = 1 as libc::c_int;
    while i < n {
        *y
            .offset(
                i as isize,
            ) = rk4(
            Some(
                rate
                    as unsafe extern "C" fn(
                        libc::c_double,
                        libc::c_double,
                    ) -> libc::c_double,
            ),
            dx,
            x0 + dx * (i - 1 as libc::c_int) as libc::c_double,
            *y.offset((i - 1 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
    printf(b"x\ty\trel. err.\n------------\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < n {
        x = x0 + dx * i as libc::c_double;
        y2 = pow(
            x * x / 4 as libc::c_int as libc::c_double
                + 1 as libc::c_int as libc::c_double,
            2 as libc::c_int as libc::c_double,
        );
        printf(
            b"%g\t%g\t%g\n\0" as *const u8 as *const libc::c_char,
            x,
            *y.offset(i as isize),
            *y.offset(i as isize) / y2 - 1 as libc::c_int as libc::c_double,
        );
        i += 10 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

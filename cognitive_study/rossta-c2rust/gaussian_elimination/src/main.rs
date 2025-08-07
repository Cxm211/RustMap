#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn swap_row(
    mut a: *mut std::os::raw::c_double,
    mut b: *mut std::os::raw::c_double,
    mut r1: std::os::raw::c_int,
    mut r2: std::os::raw::c_int,
    mut n: std::os::raw::c_int,
) {
    let mut tmp: std::os::raw::c_double = 0.;
    let mut p1: *mut std::os::raw::c_double = 0 as *mut std::os::raw::c_double;
    let mut p2: *mut std::os::raw::c_double = 0 as *mut std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    if r1 == r2 {
        return;
    }
    i = 0 as std::os::raw::c_int;
    while i < n {
        p1 = a.offset((r1 * n + i) as isize);
        p2 = a.offset((r2 * n + i) as isize);
        tmp = *p1;
        *p1 = *p2;
        *p2 = tmp;
        i += 1;
        i;
    }
    tmp = *b.offset(r1 as isize);
    *b.offset(r1 as isize) = *b.offset(r2 as isize);
    *b.offset(r2 as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn gauss_eliminate(
    mut a: *mut std::os::raw::c_double,
    mut b: *mut std::os::raw::c_double,
    mut x: *mut std::os::raw::c_double,
    mut n: std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut col: std::os::raw::c_int = 0;
    let mut row: std::os::raw::c_int = 0;
    let mut max_row: std::os::raw::c_int = 0;
    let mut dia: std::os::raw::c_int = 0;
    let mut max: std::os::raw::c_double = 0.;
    let mut tmp: std::os::raw::c_double = 0.;
    dia = 0 as std::os::raw::c_int;
    while dia < n {
        max_row = dia;
        max = *a.offset((dia * n + dia) as isize);
        row = dia + 1 as std::os::raw::c_int;
        while row < n {
            tmp = fabs(*a.offset((row * n + dia) as isize));
            if tmp > max {
                max_row = row;
                max = tmp;
            }
            row += 1;
            row;
        }
        swap_row(a, b, dia, max_row, n);
        row = dia + 1 as std::os::raw::c_int;
        while row < n {
            tmp = *a.offset((row * n + dia) as isize)
                / *a.offset((dia * n + dia) as isize);
            col = dia + 1 as std::os::raw::c_int;
            while col < n {
                *a.offset((row * n + col) as isize)
                    -= tmp * *a.offset((dia * n + col) as isize);
                col += 1;
                col;
            }
            *a.offset((row * n + dia) as isize) = 0 as std::os::raw::c_int as std::os::raw::c_double;
            *b.offset(row as isize) -= tmp * *b.offset(dia as isize);
            row += 1;
            row;
        }
        dia += 1;
        dia;
    }
    row = n - 1 as std::os::raw::c_int;
    while row >= 0 as std::os::raw::c_int {
        tmp = *b.offset(row as isize);
        j = n - 1 as std::os::raw::c_int;
        while j > row {
            tmp -= *x.offset(j as isize) * *a.offset((row * n + j) as isize);
            j -= 1;
            j;
        }
        *x.offset(row as isize) = tmp / *a.offset((row * n + row) as isize);
        row -= 1;
        row;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut a: [std::os::raw::c_double; 36] = [
        1.00f64,
        0.00f64,
        0.00f64,
        0.00f64,
        0.00f64,
        0.00f64,
        1.00f64,
        0.63f64,
        0.39f64,
        0.25f64,
        0.16f64,
        0.10f64,
        1.00f64,
        1.26f64,
        1.58f64,
        1.98f64,
        2.49f64,
        3.13f64,
        1.00f64,
        1.88f64,
        3.55f64,
        6.70f64,
        12.62f64,
        23.80f64,
        1.00f64,
        2.51f64,
        6.32f64,
        15.88f64,
        39.90f64,
        100.28f64,
        1.00f64,
        3.14f64,
        9.87f64,
        31.01f64,
        97.41f64,
        306.02f64,
    ];
    let mut b: [std::os::raw::c_double; 6] = [
        -0.01f64,
        0.61f64,
        0.91f64,
        0.99f64,
        0.60f64,
        0.02f64,
    ];
    let mut x: [std::os::raw::c_double; 6] = [0.; 6];
    let mut i: std::os::raw::c_int = 0;
    gauss_eliminate(a.as_mut_ptr(), b.as_mut_ptr(), x.as_mut_ptr(), 6 as std::os::raw::c_int);
    i = 0 as std::os::raw::c_int;
    while i < 6 as std::os::raw::c_int {
        printf(b"%g\n\0" as *const u8 as *const std::os::raw::c_char, x[i as usize]);
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

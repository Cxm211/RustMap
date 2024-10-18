#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn fabs(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn swap_row(mut a: *mut f64, mut b: *mut f64, mut r1: i32, mut r2: i32, mut n: i32) {
    unsafe {
        let mut tmp: f64 = 0.;
        let mut p1: *mut f64 = 0 as *mut f64;
        let mut p2: *mut f64 = 0 as *mut f64;
        let mut i: i32 = 0;
        if r1 == r2 {
            return;
        }
        i = 0;
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
}

#[no_mangle]
pub extern "C" fn gauss_eliminate(mut a: *mut f64, mut b: *mut f64, mut x: *mut f64, mut n: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut col: i32 = 0;
        let mut row: i32 = 0;
        let mut max_row: i32 = 0;
        let mut dia: i32 = 0;
        let mut max: f64 = 0.;
        let mut tmp: f64 = 0.;
        dia = 0;
        while dia < n {
            max_row = dia;
            max = *a.offset((dia * n + dia) as isize);
            row = dia + 1;
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
            row = dia + 1;
            while row < n {
                tmp = *a.offset((row * n + dia) as isize) / *a.offset((dia * n + dia) as isize);
                col = dia + 1;
                while col < n {
                    *a.offset((row * n + col) as isize) -=
                        tmp * *a.offset((dia * n + col) as isize);
                    col += 1;
                    col;
                }
                *a.offset((row * n + dia) as isize) = 0 as f64;
                *b.offset(row as isize) -= tmp * *b.offset(dia as isize);
                row += 1;
                row;
            }
            dia += 1;
            dia;
        }
        row = n - 1;
        while row >= 0 {
            tmp = *b.offset(row as isize);
            j = n - 1;
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
}

fn main_0() -> i32 {
    let mut a: [f64; 36] = [
        1.00f64, 0.00f64, 0.00f64, 0.00f64, 0.00f64, 0.00f64, 1.00f64, 0.63f64, 0.39f64, 0.25f64,
        0.16f64, 0.10f64, 1.00f64, 1.26f64, 1.58f64, 1.98f64, 2.49f64, 3.13f64, 1.00f64, 1.88f64,
        3.55f64, 6.70f64, 12.62f64, 23.80f64, 1.00f64, 2.51f64, 6.32f64, 15.88f64, 39.90f64,
        100.28f64, 1.00f64, 3.14f64, 9.87f64, 31.01f64, 97.41f64, 306.02f64,
    ];
    let mut b: [f64; 6] = [-0.01f64, 0.61f64, 0.91f64, 0.99f64, 0.60f64, 0.02f64];
    let mut x: [f64; 6] = [0.; 6];
    let mut i: i32 = 0;
    gauss_eliminate(a.as_mut_ptr(), b.as_mut_ptr(), x.as_mut_ptr(), 6);
    i = 0;
    while i < 6 {
        print!("{}\n", x[i as usize]);
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fabs(_: f64) -> f64;
}
pub type mat = *mut *mut f64;
#[no_mangle]
pub extern "C" fn mat_zero(mut x: mat, mut n: i32) {
    let mut i: i32 = 0;
    unsafe {
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                *(*x.offset(i as isize)).offset(j as isize) = 0 as f64;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn mat_new(mut n: i32) -> mat {
    unsafe {
        let mut x: mat =
            malloc((::core::mem::size_of::<*mut f64>() as u64).wrapping_mul(n as u64)) as mat;
        let ref mut fresh0 = *x.offset(0 as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<f64>() as u64)
                .wrapping_mul(n as u64)
                .wrapping_mul(n as u64),
        ) as *mut f64;
        let mut i: i32 = 0;
        while i < n {
            let ref mut fresh1 = *x.offset(i as isize);
            *fresh1 = (*x.offset(0 as isize)).offset((n * i) as isize);
            i += 1;
            i;
        }
        mat_zero(x, n);
        return x;
    }
}

#[no_mangle]
pub extern "C" fn mat_copy(mut s: *mut libc::c_void, mut n: i32) -> mat {
    unsafe {
        let mut x: mat = mat_new(n);
        let mut i: i32 = 0;
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                let vla = n as usize;
                *(*x.offset(i as isize)).offset(j as isize) = *(s as *mut f64)
                    .offset(i as isize * vla as isize)
                    .offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        return x;
    }
}

#[no_mangle]
pub extern "C" fn mat_del(mut x: mat) {
    unsafe {
        free(*x.offset(0 as isize) as *mut libc::c_void);
        free(x as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn mat_show(mut x: mat, mut fmt: *mut i8, mut n: i32) {
    unsafe {
        if fmt.is_null() {
            fmt = b"%8.4g\0" as *const u8 as *const i8 as *mut i8;
        }
        let mut i: i32 = 0;
        while i < n {
            if i != 0 {
                print!("      ")
            } else {
                print!(" [ ")
            };
            let mut j: i32 = 0;
            while j < n {
                printf(fmt, *(*x.offset(i as isize)).offset(j as isize));
                if j < n - 1 {
                    print!("  ")
                } else {
                    if i == n - 1 {
                        print!(" ]\n")
                    } else {
                        print!("\n")
                    }
                };
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn mat_mul(mut a: mat, mut b: mat, mut n: i32) -> mat {
    let mut c: mat = 0 as *mut *mut f64;
    c = mat_new(n);
    c = c;
    let mut i: i32 = 0;
    unsafe {
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                let mut k: i32 = 0;
                while k < n {
                    *(*c.offset(i as isize)).offset(j as isize) += *(*a.offset(i as isize))
                        .offset(k as isize)
                        * *(*b.offset(k as isize)).offset(j as isize);
                    k += 1;
                    k;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    return c;
}

#[no_mangle]
pub extern "C" fn mat_pivot(mut a: mat, mut p: mat, mut n: i32) {
    let mut i: i32 = 0;
    unsafe {
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                *(*p.offset(i as isize)).offset(j as isize) = (i == j) as i32 as f64;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    let mut i_0: i32 = 0;
    unsafe {
        while i_0 < n {
            let mut max_j: i32 = i_0;
            let mut j_0: i32 = i_0;
            while j_0 < n {
                if fabs(*(*a.offset(j_0 as isize)).offset(i_0 as isize))
                    > fabs(*(*a.offset(max_j as isize)).offset(i_0 as isize))
                {
                    max_j = j_0;
                }
                j_0 += 1;
                j_0;
            }
            if max_j != i_0 {
                let mut k: i32 = 0;
                while k < n {
                    let mut tmp: f64 = *(*p.offset(i_0 as isize)).offset(k as isize);
                    *(*p.offset(i_0 as isize)).offset(k as isize) =
                        *(*p.offset(max_j as isize)).offset(k as isize);
                    *(*p.offset(max_j as isize)).offset(k as isize) = tmp;
                    k += 1;
                    k;
                }
            }
            i_0 += 1;
            i_0;
        }
    }
}

#[no_mangle]
pub extern "C" fn mat_LU(mut A: mat, mut L: mat, mut U: mat, mut P: mat, mut n: i32) {
    mat_zero(L, n);
    mat_zero(U, n);
    mat_pivot(A, P, n);
    let mut Aprime: mat = mat_mul(P, A, n);
    let mut i: i32 = 0;
    unsafe {
        while i < n {
            *(*L.offset(i as isize)).offset(i as isize) = 1 as f64;
            i += 1;
            i;
        }
    }
    let mut i_0: i32 = 0;
    unsafe {
        while i_0 < n {
            let mut j: i32 = 0;
            while j < n {
                let mut s: f64 = 0.;
                if j <= i_0 {
                    s = 0 as f64;
                    let mut k: i32 = 0;
                    while k < j {
                        s += *(*L.offset(j as isize)).offset(k as isize)
                            * *(*U.offset(k as isize)).offset(i_0 as isize);
                        k += 1;
                        k;
                    }
                    *(*U.offset(j as isize)).offset(i_0 as isize) =
                        *(*Aprime.offset(j as isize)).offset(i_0 as isize) - s;
                }
                if j >= i_0 {
                    s = 0 as f64;
                    let mut k_0: i32 = 0;
                    while k_0 < i_0 {
                        s += *(*L.offset(j as isize)).offset(k_0 as isize)
                            * *(*U.offset(k_0 as isize)).offset(i_0 as isize);
                        k_0 += 1;
                        k_0;
                    }
                    *(*L.offset(j as isize)).offset(i_0 as isize) =
                        (*(*Aprime.offset(j as isize)).offset(i_0 as isize) - s)
                            / *(*U.offset(i_0 as isize)).offset(i_0 as isize);
                }
                j += 1;
                j;
            }
            i_0 += 1;
            i_0;
        }
    }
    mat_del(Aprime);
}

#[no_mangle]
pub static mut A3: [[f64; 3]; 3] = [
    [1 as f64, 3 as f64, 5 as f64],
    [2 as f64, 4 as f64, 7 as f64],
    [1 as f64, 1 as f64, 0 as f64],
];
#[no_mangle]
pub static mut A4: [[f64; 4]; 4] = [
    [11 as f64, 9 as f64, 24 as f64, 2 as f64],
    [1 as f64, 5 as f64, 2 as f64, 6 as f64],
    [3 as f64, 17 as f64, 18 as f64, 1 as f64],
    [2 as f64, 5 as f64, 7 as f64, 1 as f64],
];
fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut A: mat = 0 as *mut *mut f64;
    let mut L: mat = 0 as *mut *mut f64;
    let mut P: mat = 0 as *mut *mut f64;
    let mut U: mat = 0 as *mut *mut f64;
    L = mat_new(n);
    P = mat_new(n);
    U = mat_new(n);
    unsafe {
        A = mat_copy(A3.as_mut_ptr() as *mut libc::c_void, n);
    }
    mat_LU(A, L, U, P, n);
    print!("A =");
    mat_show(A, 0 as *mut i8, n);
    print!("L =");
    mat_show(L, 0 as *mut i8, n);
    print!("U =");
    mat_show(U, 0 as *mut i8, n);
    print!("P =");
    mat_show(P, 0 as *mut i8, n);
    mat_del(A);
    mat_del(L);
    mat_del(U);
    mat_del(P);
    print!("\n");
    n = 4;
    L = mat_new(n);
    P = mat_new(n);
    U = mat_new(n);
    unsafe {
        A = mat_copy(A4.as_mut_ptr() as *mut libc::c_void, n);
    }
    mat_LU(A, L, U, P, n);
    print!("A =");
    mat_show(A, 0 as *mut i8, n);
    print!("L =");
    mat_show(L, 0 as *mut i8, n);
    print!("U =");
    mat_show(U, 0 as *mut i8, n);
    print!("P =");
    mat_show(P, 0 as *mut i8, n);
    mat_del(A);
    mat_del(L);
    mat_del(U);
    mat_del(P);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

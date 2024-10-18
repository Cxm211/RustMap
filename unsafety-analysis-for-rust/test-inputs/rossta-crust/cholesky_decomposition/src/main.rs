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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn cholesky(mut A: *mut f64, mut n: i32) -> *mut f64 {
    unsafe {
        let mut L: *mut f64 =
            calloc((n * n) as u64, ::core::mem::size_of::<f64>() as u64) as *mut f64;
        if L.is_null() {
            exit(1);
        }
        let mut i: i32 = 0;
        while i < n {
            let mut j: i32 = 0;
            while j < i + 1 {
                let mut s: f64 = 0 as f64;
                let mut k: i32 = 0;
                while k < j {
                    s += *L.offset((i * n + k) as isize) * *L.offset((j * n + k) as isize);
                    k += 1;
                    k;
                }
                *L.offset((i * n + j) as isize) = if i == j {
                    sqrt(*A.offset((i * n + i) as isize) - s)
                } else {
                    1.0f64 / *L.offset((j * n + j) as isize) * (*A.offset((i * n + j) as isize) - s)
                };
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        return L;
    }
}

#[no_mangle]
pub extern "C" fn show_matrix(mut A: *mut f64, mut n: i32) {
    unsafe {
        let mut i: i32 = 0;
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                print!("{:2.5} ", *A.offset((i * n + j) as isize));
                j += 1;
                j;
            }
            print!("\n");
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut n: i32 = 3;
        let mut m1: [f64; 9] = [
            25 as f64,
            15 as f64,
            -5i32 as f64,
            15 as f64,
            18 as f64,
            0 as f64,
            -5i32 as f64,
            0 as f64,
            11 as f64,
        ];
        let mut c1: *mut f64 = cholesky(m1.as_mut_ptr(), n);
        show_matrix(c1, n);
        print!("\n");
        free(c1 as *mut libc::c_void);
        n = 4;
        let mut m2: [f64; 16] = [
            18 as f64, 22 as f64, 54 as f64, 42 as f64, 22 as f64, 70 as f64, 86 as f64, 62 as f64,
            54 as f64, 86 as f64, 174 as f64, 134 as f64, 42 as f64, 62 as f64, 134 as f64,
            106 as f64,
        ];
        let mut c2: *mut f64 = cholesky(m2.as_mut_ptr(), n);
        show_matrix(c2, n);
        free(c2 as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}

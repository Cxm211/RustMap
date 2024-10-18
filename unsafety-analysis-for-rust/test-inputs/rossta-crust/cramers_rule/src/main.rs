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
    fn puts(__s: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: i32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SquareMatrix {
    pub n: i32,
    pub elems: *mut *mut f64,
}
#[no_mangle]
pub extern "C" fn init_square_matrix(mut n: i32, mut elems: *mut f64) -> SquareMatrix {
    unsafe {
        let vla = n as usize;
        let mut A: SquareMatrix = {
            let mut init = SquareMatrix {
                n: n,
                elems: malloc((n as u64).wrapping_mul(::core::mem::size_of::<*mut f64>() as u64))
                    as *mut *mut f64,
            };
            init
        };
        let mut i: i32 = 0;
        while i < n {
            let ref mut fresh0 = *(A.elems).offset(i as isize);
            *fresh0 =
                malloc((n as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64)) as *mut f64;
            let mut j: i32 = 0;
            while j < n {
                *(*(A.elems).offset(i as isize)).offset(j as isize) =
                    *elems.offset(i as isize * vla as isize).offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        return A;
    }
}

#[no_mangle]
pub extern "C" fn copy_square_matrix(mut src: SquareMatrix) -> SquareMatrix {
    let mut dest: SquareMatrix = SquareMatrix {
        n: 0,
        elems: 0 as *mut *mut f64,
    };
    dest.n = src.n;
    unsafe {
        dest.elems = malloc((dest.n as u64).wrapping_mul(::core::mem::size_of::<*mut f64>() as u64))
            as *mut *mut f64;
    }
    let mut i: i32 = 0;
    unsafe {
        while i < dest.n {
            let ref mut fresh1 = *(dest.elems).offset(i as isize);
            *fresh1 = malloc((dest.n as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64))
                as *mut f64;
            let mut j: i32 = 0;
            while j < dest.n {
                *(*(dest.elems).offset(i as isize)).offset(j as isize) =
                    *(*(src.elems).offset(i as isize)).offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    return dest;
}

#[no_mangle]
pub extern "C" fn det(mut A: SquareMatrix) -> f64 {
    let mut det_0: f64 = 1 as f64;
    let mut j: i32 = 0;
    unsafe {
        while j < A.n {
            let mut i_max: i32 = j;
            let mut i: i32 = j;
            while i < A.n {
                if *(*(A.elems).offset(i as isize)).offset(j as isize)
                    > *(*(A.elems).offset(i_max as isize)).offset(j as isize)
                {
                    i_max = i;
                }
                i += 1;
                i;
            }
            if i_max != j {
                let mut k: i32 = 0;
                while k < A.n {
                    let mut tmp: f64 = *(*(A.elems).offset(i_max as isize)).offset(k as isize);
                    *(*(A.elems).offset(i_max as isize)).offset(k as isize) =
                        *(*(A.elems).offset(j as isize)).offset(k as isize);
                    *(*(A.elems).offset(j as isize)).offset(k as isize) = tmp;
                    k += 1;
                    k;
                }
                det_0 *= -1i32 as f64;
            }
            if (abs(*(*(A.elems).offset(j as isize)).offset(j as isize) as i32) as f64) < 1e-12f64 {
                puts(b"Singular matrix!\0" as *const u8 as *const i8);
                return ::core::f32::NAN as f64;
            }
            let mut i_0: i32 = j + 1;
            while i_0 < A.n {
                let mut mult: f64 = -*(*(A.elems).offset(i_0 as isize)).offset(j as isize)
                    / *(*(A.elems).offset(j as isize)).offset(j as isize);
                let mut k_0: i32 = 0;
                while k_0 < A.n {
                    *(*(A.elems).offset(i_0 as isize)).offset(k_0 as isize) +=
                        mult * *(*(A.elems).offset(j as isize)).offset(k_0 as isize);
                    k_0 += 1;
                    k_0;
                }
                i_0 += 1;
                i_0;
            }
            j += 1;
            j;
        }
    }
    let mut i_1: i32 = 0;
    unsafe {
        while i_1 < A.n {
            det_0 *= *(*(A.elems).offset(i_1 as isize)).offset(i_1 as isize);
            i_1 += 1;
            i_1;
        }
    }
    return det_0;
}

#[no_mangle]
pub extern "C" fn deinit_square_matrix(mut A: SquareMatrix) {
    let mut i: i32 = 0;
    unsafe {
        while i < A.n {
            free(*(A.elems).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        free(A.elems as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn cramer_solve(
    mut A: SquareMatrix,
    mut det_A: f64,
    mut b: *mut f64,
    mut var: i32,
) -> f64 {
    unsafe {
        let mut tmp: SquareMatrix = copy_square_matrix(A);
        let mut i: i32 = 0;
        while i < tmp.n {
            *(*(tmp.elems).offset(i as isize)).offset(var as isize) = *b.offset(i as isize);
            i += 1;
            i;
        }
        let mut det_tmp: f64 = det(tmp);
        deinit_square_matrix(tmp);
        return det_tmp / det_A;
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut elems: [[f64; 4]; 4] = [
            [2 as f64, -1i32 as f64, 5 as f64, 1 as f64],
            [3 as f64, 2 as f64, 2 as f64, -6i32 as f64],
            [1 as f64, 3 as f64, 3 as f64, -1i32 as f64],
            [5 as f64, -2i32 as f64, -3i32 as f64, 3 as f64],
        ];
        let mut A: SquareMatrix = init_square_matrix(4, elems.as_mut_ptr() as *mut f64);
        let mut tmp: SquareMatrix = copy_square_matrix(A);
        let mut det_A: i32 = det(tmp) as i32;
        deinit_square_matrix(tmp);
        let mut b: [f64; 4] = [-3i32 as f64, -32i32 as f64, -47i32 as f64, 49 as f64];
        let mut i: i32 = 0;
        while i < 4 {
            print!("{:7.3}\n", cramer_solve(A, det_A as f64, b.as_mut_ptr(), i));
            i += 1;
            i;
        }
        deinit_square_matrix(A);
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}

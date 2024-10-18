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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub static mut m: *mut *mut i32 = 0 as *const *mut i32 as *mut *mut i32;
#[no_mangle]
pub static mut s: *mut *mut i32 = 0 as *const *mut i32 as *mut *mut i32;
#[no_mangle]
pub extern "C" fn optimal_matrix_chain_order(mut dims: *mut i32, mut n: i32) {
    unsafe {
        let mut len: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut temp: i32 = 0;
        let mut cost: i32 = 0;
        n -= 1;
        n;
        m = malloc((n as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64))
            as *mut *mut i32;
        i = 0;
        while i < n {
            let ref mut fresh0 = *m.offset(i as isize);
            *fresh0 = calloc(n as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
            i += 1;
            i;
        }
        s = malloc((n as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64))
            as *mut *mut i32;
        i = 0;
        while i < n {
            let ref mut fresh1 = *s.offset(i as isize);
            *fresh1 = calloc(n as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
            i += 1;
            i;
        }
        len = 1;
        while len < n {
            i = 0;
            while i < n - len {
                j = i + len;
                *(*m.offset(i as isize)).offset(j as isize) = 2147483647;
                k = i;
                while k < j {
                    temp = *dims.offset(i as isize)
                        * *dims.offset((k + 1i32) as isize)
                        * *dims.offset((j + 1i32) as isize);
                    cost = *(*m.offset(i as isize)).offset(k as isize)
                        + *(*m.offset((k + 1i32) as isize)).offset(j as isize)
                        + temp;
                    if cost < *(*m.offset(i as isize)).offset(j as isize) {
                        *(*m.offset(i as isize)).offset(j as isize) = cost;
                        *(*s.offset(i as isize)).offset(j as isize) = k;
                    }
                    k += 1;
                    k;
                }
                i += 1;
                i;
            }
            len += 1;
            len;
        }
    }
}

#[no_mangle]
pub extern "C" fn print_optimal_chain_order(mut i: i32, mut j: i32) {
    unsafe {
        if i == j {
            print!("{}", i + 65);
        } else {
            print!("(");
            print_optimal_chain_order(i, *(*s.offset(i as isize)).offset(j as isize));
            print_optimal_chain_order(*(*s.offset(i as isize)).offset(j as isize) + 1, j);
            print!(")");
        };
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut a1: [i32; 4] = [5, 6, 3, 1];
    let mut a2: [i32; 13] = [1, 5, 25, 30, 100, 70, 2, 1, 100, 250, 1, 1000, 2];
    let mut a3: [i32; 12] = [1000, 1, 500, 12, 1, 700, 2500, 3, 2, 5, 14, 10];
    let mut dims_list: [*mut i32; 3] = [a1.as_mut_ptr(), a2.as_mut_ptr(), a3.as_mut_ptr()];
    let mut sizes: [i32; 3] = [4, 13, 12];
    i = 0;
    unsafe {
        while i < 3 {
            print!("Dims  : [");
            n = sizes[i as usize];
            j = 0;
            while j < n {
                print!("{}", *(dims_list[i as usize]).offset(j as isize));
                if j < n - 1 {
                    print!(", ");
                } else {
                    print!("]\n");
                }
                j += 1;
                j;
            }
            optimal_matrix_chain_order(dims_list[i as usize], n);
            print!("Order : ");
            print_optimal_chain_order(0, n - 2);
            print!(
                "\nCost  : {}\n\n",
                *(*m.offset(0 as isize)).offset((n - 2i32) as isize)
            );
            j = 0;
            while j <= n - 2 {
                free(*m.offset(j as isize) as *mut libc::c_void);
                j += 1;
                j;
            }
            free(m as *mut libc::c_void);
            j = 0;
            while j <= n - 2 {
                free(*s.offset(j as isize) as *mut libc::c_void);
                j += 1;
                j;
            }
            free(s as *mut libc::c_void);
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

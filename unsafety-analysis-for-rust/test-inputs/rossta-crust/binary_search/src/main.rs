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
extern "C" {}
#[no_mangle]
pub extern "C" fn bsearch(mut a: *mut i32, mut n: i32, mut x: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = n - 1;
        while i <= j {
            let mut k: i32 = i + (j - i) / 2;
            if *a.offset(k as isize) == x {
                return k;
            } else if *a.offset(k as isize) < x {
                i = k + 1;
            } else {
                j = k - 1;
            }
        }
        return -1;
    }
}

#[no_mangle]
pub extern "C" fn bsearch_r(mut a: *mut i32, mut x: i32, mut i: i32, mut j: i32) -> i32 {
    unsafe {
        if j < i {
            return -1;
        }
        let mut k: i32 = i + (j - i) / 2;
        if *a.offset(k as isize) == x {
            return k;
        } else if *a.offset(k as isize) < x {
            return bsearch_r(a, x, k + 1, j);
        } else {
            return bsearch_r(a, x, i, k - 1);
        };
    }
}

fn main_0() -> i32 {
    let mut a: [i32; 10] = [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
    let mut n: i32 = (::core::mem::size_of::<[i32; 10]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    let mut x: i32 = 2;
    let mut i: i32 = bsearch(a.as_mut_ptr(), n, x);
    print!("{} is at index {}\n", x, i);
    x = 5;
    i = bsearch_r(a.as_mut_ptr(), x, 0, n - 1);
    print!("{} is at index {}\n", x, i);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

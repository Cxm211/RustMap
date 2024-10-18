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
pub extern "C" fn mul_inv(mut a: i32, mut b: i32) -> i32 {
    let mut b0: i32 = b;
    let mut t: i32 = 0;
    let mut q: i32 = 0;
    let mut x0: i32 = 0;
    let mut x1: i32 = 1;
    if b == 1 {
        return 1;
    }
    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 += b0;
    }
    return x1;
}

#[no_mangle]
pub extern "C" fn chinese_remainder(mut n: *mut i32, mut a: *mut i32, mut len: i32) -> i32 {
    unsafe {
        let mut p: i32 = 0;
        let mut i: i32 = 0;
        let mut prod: i32 = 1;
        let mut sum: i32 = 0;
        i = 0;
        while i < len {
            prod *= *n.offset(i as isize);
            i += 1;
            i;
        }
        i = 0;
        while i < len {
            p = prod / *n.offset(i as isize);
            sum += *a.offset(i as isize) * mul_inv(p, *n.offset(i as isize)) * p;
            i += 1;
            i;
        }
        return sum % prod;
    }
}

fn main_0() -> i32 {
    let mut n: [i32; 3] = [3, 5, 7];
    let mut a: [i32; 3] = [2, 3, 2];
    print!(
        "{}\n",
        chinese_remainder(
            n.as_mut_ptr(),
            a.as_mut_ptr(),
            (::core::mem::size_of::<[i32; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
        )
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

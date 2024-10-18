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
    fn rand() -> i32;
}
#[no_mangle]
pub extern "C" fn irand(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut randmax: i32 = 2147483647 / n * n;
    unsafe {
        loop {
            r = rand();
            if !(r >= randmax) {
                break;
            }
        }
    }
    return r / (randmax / n);
}

#[no_mangle]
pub extern "C" fn one_of_n(mut n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    i = 1;
    while i < n {
        if irand(i + 1) == 0 {
            r = i;
        }
        i += 1;
        i;
    }
    return r;
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut r: [i32; 10] = [0; 10];
    i = 0;
    while i < 1000000 {
        i += 1;
        i;
        r[one_of_n(10) as usize] += 1;
        r[one_of_n(10) as usize];
    }
    i = 0;
    while i < 10 {
        if i == 9 {
            print!("{}{}", r[i as usize], '\n' as i32)
        } else {
            print!("{}{}", r[i as usize], ' ' as i32)
        };
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

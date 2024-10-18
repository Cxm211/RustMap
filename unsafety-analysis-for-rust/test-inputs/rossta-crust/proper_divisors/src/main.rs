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
pub extern "C" fn proper_divisors(n: i32, mut print_flag: bool) -> i32 {
    let mut count: i32 = 0;
    let mut i: i32 = 1;
    while i < n {
        if n % i == 0 {
            count += 1;
            count;
            if print_flag {
                print!("{} ", i);
            }
        }
        i += 1;
        i;
    }
    if print_flag {
        print!("\n");
    }
    return count;
}

#[no_mangle]
pub extern "C" fn countProperDivisors(mut n: i32) -> i32 {
    let mut prod: i32 = 1;
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    while n % 2 == 0 {
        count += 1;
        count;
        n /= 2;
    }
    prod *= 1 + count;
    i = 3;
    while i * i <= n {
        count = 0;
        while n % i == 0 {
            count += 1;
            count;
            n /= i;
        }
        prod *= 1 + count;
        i += 2;
    }
    if n > 2 {
        prod *= 2;
    }
    return prod - 1;
}

fn main_0() -> i32 {
    let mut i: i32 = 1;
    while i <= 10 {
        print!("{}: ", i);
        proper_divisors(i, 1 != 0);
        i += 1;
        i;
    }
    let mut max: i32 = 0;
    let mut max_i: i32 = 1;
    let mut i_0: i32 = 1;
    while i_0 <= 20000 {
        let mut v: i32 = countProperDivisors(i_0);
        if v >= max {
            max = v;
            max_i = i_0;
        }
        i_0 += 1;
        i_0;
    }
    print!("{} with {} divisors\n", max_i, max);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

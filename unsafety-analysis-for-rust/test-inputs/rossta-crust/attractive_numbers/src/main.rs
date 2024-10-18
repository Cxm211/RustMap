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
pub extern "C" fn is_prime(mut n: i32) -> i32 {
    let mut d: i32 = 5;
    if n < 2 {
        return 0;
    }
    if n % 2 == 0 {
        return (n == 2) as i32;
    }
    if n % 3 == 0 {
        return (n == 3) as i32;
    }
    while d * d <= n {
        if n % d == 0 {
            return 0;
        }
        d += 2;
        if n % d == 0 {
            return 0;
        }
        d += 4;
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn count_prime_factors(mut n: i32) -> i32 {
    let mut count: i32 = 0;
    let mut f: i32 = 2;
    if n == 1 {
        return 0;
    }
    if is_prime(n) != 0 {
        return 1;
    }
    loop {
        if n % f == 0 {
            count += 1;
            count;
            n /= f;
            if n == 1 {
                return count;
            }
            if is_prime(n) != 0 {
                f = n;
            }
        } else if f >= 3 {
            f += 2;
        } else {
            f = 3;
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut count: i32 = 0;
    print!("The attractive numbers up to and including {} are:\n", 120);
    i = 1;
    while i <= 120 {
        n = count_prime_factors(i);
        if is_prime(n) != 0 {
            print!("{:4}", i);
            count += 1;
            if count % 20 == 0 {
                print!("\n");
            }
        }
        i += 1;
        i;
    }
    print!("\n");
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

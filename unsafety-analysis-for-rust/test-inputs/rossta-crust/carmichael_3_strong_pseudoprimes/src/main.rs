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
pub extern "C" fn is_prime(mut n: u32) -> i32 {
    if n <= 3 {
        return (n > 1) as i32;
    } else if n.wrapping_rem(2) == 0 || n.wrapping_rem(3) == 0 {
        return 0;
    } else {
        let mut i: u32 = 0;
        i = 5;
        while i.wrapping_mul(i) <= n {
            if n.wrapping_rem(i) == 0 || n.wrapping_rem(i.wrapping_add(2)) == 0 {
                return 0;
            }
            i = i.wrapping_add(6);
        }
        return 1;
    };
}

#[no_mangle]
pub extern "C" fn carmichael3(mut p1: i32) {
    if is_prime(p1 as u32) == 0 {
        return;
    }
    let mut h3: i32 = 0;
    let mut d: i32 = 0;
    let mut p2: i32 = 0;
    let mut p3: i32 = 0;
    h3 = 1;
    while h3 < p1 {
        d = 1;
        while d < h3 + p1 {
            if (h3 + p1) * (p1 - 1) % d == 0 && (-p1 * p1 % h3 + h3) % h3 == d % h3 {
                p2 = 1 + (p1 - 1) * (h3 + p1) / d;
                if !(is_prime(p2 as u32) == 0) {
                    p3 = 1 + p1 * p2 / h3;
                    if !(is_prime(p3 as u32) == 0 || p2 * p3 % (p1 - 1) != 1) {
                        print!("{} {} {}\n", p1, p2, p3);
                    }
                }
            }
            d += 1;
            d;
        }
        h3 += 1;
        h3;
    }
}

fn main_0() -> i32 {
    let mut p1: i32 = 0;
    p1 = 2;
    while p1 < 62 {
        carmichael3(p1);
        p1 += 1;
        p1;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

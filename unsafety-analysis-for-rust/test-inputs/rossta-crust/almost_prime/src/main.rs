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
pub extern "C" fn kprime(mut n: i32, mut k: i32) -> i32 {
    let mut p: i32 = 0;
    let mut f: i32 = 0;
    p = 2;
    while f < k && p * p <= n {
        while 0 == n % p {
            n /= p;
            f += 1;
            f;
        }
        p += 1;
        p;
    }
    return (f + (n > 1i32) as i32 == k) as i32;
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut k: i32 = 0;
    k = 1;
    while k <= 5 {
        print!("k = {}:", k);
        i = 2;
        c = 0;
        while c < 10 {
            if kprime(i, k) != 0 {
                print!(" {}", i);
                c += 1;
                c;
            }
            i += 1;
            i;
        }
        print!("{}", '\n' as i32);
        k += 1;
        k;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

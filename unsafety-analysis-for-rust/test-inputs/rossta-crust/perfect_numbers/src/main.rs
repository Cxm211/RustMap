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
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn perfect(mut n: i32) -> i32 {
    unsafe {
        let mut max: i32 = sqrt(n as f64) as i32 + 1;
        let mut tot: i32 = 1;
        let mut i: i32 = 0;
        i = 2;
        while i < max {
            if n % i == 0 {
                tot += i;
                let mut q: i32 = n / i;
                if q > i {
                    tot += q;
                }
            }
            i += 1;
            i;
        }
        return (tot == n) as i32;
    }
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    n = 2;
    while n < 200 {
        if perfect(n) != 0 {
            print!("{}\n", n);
        }
        n += 1;
        n;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

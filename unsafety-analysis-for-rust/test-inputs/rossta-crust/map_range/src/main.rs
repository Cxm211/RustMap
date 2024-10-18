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
}
#[no_mangle]
pub extern "C" fn mapRange(mut a1: f64, mut a2: f64, mut b1: f64, mut b2: f64, mut s: f64) -> f64 {
    return b1 + (s - a1) * (b2 - b1) / (a2 - a1);
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    unsafe {
        puts(b"Mapping [0,10] to [-1,0] at intervals of 1:\0" as *const u8 as *const i8);
    }
    i = 0;
    while i <= 10 {
        print!(
            "f({}) = {}\n",
            i,
            mapRange(0 as f64, 10 as f64, -1i32 as f64, 0 as f64, i as f64)
        );
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

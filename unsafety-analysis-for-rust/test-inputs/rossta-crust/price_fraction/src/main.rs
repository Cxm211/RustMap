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
    fn abort() -> !;
}
#[no_mangle]
pub static mut table: [[f64; 2]; 21] = [
    [0.06f64, 0.10f64],
    [0.11f64, 0.18f64],
    [0.16f64, 0.26f64],
    [0.21f64, 0.32f64],
    [0.26f64, 0.38f64],
    [0.31f64, 0.44f64],
    [0.36f64, 0.50f64],
    [0.41f64, 0.54f64],
    [0.46f64, 0.58f64],
    [0.51f64, 0.62f64],
    [0.56f64, 0.66f64],
    [0.61f64, 0.70f64],
    [0.66f64, 0.74f64],
    [0.71f64, 0.78f64],
    [0.76f64, 0.82f64],
    [0.81f64, 0.86f64],
    [0.86f64, 0.90f64],
    [0.91f64, 0.94f64],
    [0.96f64, 0.98f64],
    [1.01f64, 1.00f64],
    [-1i32 as f64, 0 as f64],
];
#[no_mangle]
pub extern "C" fn price_fix(mut x: f64) -> f64 {
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while table[i as usize][0 as usize] > 0 as f64 {
            if x < table[i as usize][0 as usize] {
                return table[i as usize][1 as usize];
            }
            i += 1;
            i;
        }
        abort();
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i <= 100 {
        print!(
            "{:.2} {:.2}\n",
            i as f64 / 100.0f64,
            price_fix(i as f64 / 100.0f64)
        );
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

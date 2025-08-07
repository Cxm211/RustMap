#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn abort() -> !;
}
#[no_mangle]
pub static mut table: [[std::os::raw::c_double; 2]; 21] = [
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
    [-(1 as std::os::raw::c_int) as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double],
];
#[no_mangle]
pub unsafe extern "C" fn price_fix(mut x: std::os::raw::c_double) -> std::os::raw::c_double {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while table[i as usize][0 as std::os::raw::c_int as usize]
        > 0 as std::os::raw::c_int as std::os::raw::c_double
    {
        if x < table[i as usize][0 as std::os::raw::c_int as usize] {
            return table[i as usize][1 as std::os::raw::c_int as usize];
        }
        i += 1;
        i;
    }
    abort();
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i <= 100 as std::os::raw::c_int {
        printf(
            b"%.2f %.2f\n\0" as *const u8 as *const std::os::raw::c_char,
            i as std::os::raw::c_double / 100.0f64,
            price_fix(i as std::os::raw::c_double / 100.0f64),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

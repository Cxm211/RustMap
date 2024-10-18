#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn gcd(mut m: std::os::raw::c_int, mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut tmp: std::os::raw::c_int = 0;
    while m != 0 {
        tmp = m;
        m = n % m;
        n = tmp;
    }
    return n;
}
#[no_mangle]
pub extern "C" fn lcm(mut m: std::os::raw::c_int, mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    return m / gcd(m, n) * n;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"lcm(35, 21) = %d\n\0" as *const u8 as *const std::os::raw::c_char,
        lcm(21 as std::os::raw::c_int, 35 as std::os::raw::c_int),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

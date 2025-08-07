#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn perfect(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut max: std::os::raw::c_int = sqrt(n as std::os::raw::c_double) as std::os::raw::c_int
        + 1 as std::os::raw::c_int;
    let mut tot: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    i = 2 as std::os::raw::c_int;
    while i < max {
        if n % i == 0 as std::os::raw::c_int {
            tot += i;
            let mut q: std::os::raw::c_int = n / i;
            if q > i {
                tot += q;
            }
        }
        i += 1;
        i;
    }
    return (tot == n) as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0;
    n = 2 as std::os::raw::c_int;
    while n < 200 as std::os::raw::c_int {
        if perfect(n) != 0 {
            printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, n);
        }
        n += 1;
        n;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

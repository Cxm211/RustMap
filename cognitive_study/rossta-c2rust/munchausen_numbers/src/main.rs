#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < 5000 as libc::c_int {
        let mut sum: libc::c_int = 0 as libc::c_int;
        let mut number: libc::c_int = i;
        while number > 0 as libc::c_int {
            let mut digit: libc::c_int = number % 10 as libc::c_int;
            sum = (sum as libc::c_double
                + pow(digit as libc::c_double, digit as libc::c_double)) as libc::c_int;
            number /= 10 as libc::c_int;
        }
        if sum == i {
            printf(b"%i\n\0" as *const u8 as *const libc::c_char, i);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

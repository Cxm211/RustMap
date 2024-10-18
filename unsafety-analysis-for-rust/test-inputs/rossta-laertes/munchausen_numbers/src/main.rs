#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while i < 5000 as std::os::raw::c_int {
        let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut number: std::os::raw::c_int = i;
        while number > 0 as std::os::raw::c_int {
            let mut digit: std::os::raw::c_int = number % 10 as std::os::raw::c_int;
            sum = (sum as std::os::raw::c_double
                + pow(digit as std::os::raw::c_double, digit as std::os::raw::c_double)) as std::os::raw::c_int;
            number /= 10 as std::os::raw::c_int;
        }
        if sum == i {
            printf(b"%i\n\0" as *const u8 as *const std::os::raw::c_char, i);
        }
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

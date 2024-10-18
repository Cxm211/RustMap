#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn semiprime(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut p: std::os::raw::c_int = 0;
    let mut f: std::os::raw::c_int = 0 as std::os::raw::c_int;
    p = 2 as std::os::raw::c_int;
    while f < 2 as std::os::raw::c_int && p * p <= n {
        while 0 as std::os::raw::c_int == n % p {
            n /= p;
            f += 1;
            f;
        }
        p += 1;
        p;
    }
    return (f + (n > 1 as std::os::raw::c_int) as std::os::raw::c_int == 2 as std::os::raw::c_int)
        as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 2 as std::os::raw::c_int;
    while i < 100 as std::os::raw::c_int {
        if semiprime(i) != 0 {
            printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, i);
        }
        i += 1;
        i;
    }
    putchar('\n' as i32);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

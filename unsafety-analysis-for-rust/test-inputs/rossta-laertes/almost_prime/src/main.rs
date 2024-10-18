#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn kprime(mut n: std::os::raw::c_int, mut k: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut p: std::os::raw::c_int = 0;
    let mut f: std::os::raw::c_int = 0 as std::os::raw::c_int;
    p = 2 as std::os::raw::c_int;
    while f < k && p * p <= n {
        while 0 as std::os::raw::c_int == n % p {
            n /= p;
            f += 1;
            f;
        }
        p += 1;
        p;
    }
    return (f + (n > 1 as std::os::raw::c_int) as std::os::raw::c_int == k) as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    k = 1 as std::os::raw::c_int;
    while k <= 5 as std::os::raw::c_int {
        printf(b"k = %d:\0" as *const u8 as *const std::os::raw::c_char, k);
        i = 2 as std::os::raw::c_int;
        c = 0 as std::os::raw::c_int;
        while c < 10 as std::os::raw::c_int {
            if kprime(i, k) != 0 {
                printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, i);
                c += 1;
                c;
            }
            i += 1;
            i;
        }
        putchar('\n' as i32);
        k += 1;
        k;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

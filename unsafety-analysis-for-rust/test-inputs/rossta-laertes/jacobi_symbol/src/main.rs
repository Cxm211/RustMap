#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn jacobi(
    mut a: std::os::raw::c_ulong,
    mut n: std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    if a >= n {
        a = a.wrapping_rem(n);
    }
    let mut result: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while a != 0 {
        while a & 1 as std::os::raw::c_int as std::os::raw::c_ulong == 0 as std::os::raw::c_int as std::os::raw::c_ulong
        {
            a >>= 1 as std::os::raw::c_int;
            if n & 7 as std::os::raw::c_int as std::os::raw::c_ulong == 3 as std::os::raw::c_int as std::os::raw::c_ulong
                || n & 7 as std::os::raw::c_int as std::os::raw::c_ulong
                    == 5 as std::os::raw::c_int as std::os::raw::c_ulong
            {
                result = -result;
            }
        }
        a ^= n;
        n ^= a;
        a ^= n;
        if a & 3 as std::os::raw::c_int as std::os::raw::c_ulong == 3 as std::os::raw::c_int as std::os::raw::c_ulong
            && n & 3 as std::os::raw::c_int as std::os::raw::c_ulong == 3 as std::os::raw::c_int as std::os::raw::c_ulong
        {
            result = -result;
        }
        a = a.wrapping_rem(n);
    }
    if n == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return result;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_table(mut kmax: std::os::raw::c_uint, mut nmax: std::os::raw::c_uint) {
    printf(b"n\\k|\0" as *const u8 as *const std::os::raw::c_char);
    let mut k: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while k as std::os::raw::c_uint <= kmax {
        printf(b"%'3u\0" as *const u8 as *const std::os::raw::c_char, k);
        k += 1;
        k;
    }
    printf(b"\n----\0" as *const u8 as *const std::os::raw::c_char);
    let mut k_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while k_0 as std::os::raw::c_uint <= kmax {
        printf(b"---\0" as *const u8 as *const std::os::raw::c_char);
        k_0 += 1;
        k_0;
    }
    putchar('\n' as i32);
    let mut n: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while n as std::os::raw::c_uint <= nmax {
        printf(b"%-2u |\0" as *const u8 as *const std::os::raw::c_char, n);
        let mut k_1: std::os::raw::c_int = 0 as std::os::raw::c_int;
        while k_1 as std::os::raw::c_uint <= kmax {
            printf(
                b"%'3d\0" as *const u8 as *const std::os::raw::c_char,
                jacobi(k_1 as std::os::raw::c_ulong, n as std::os::raw::c_ulong),
            );
            k_1 += 1;
            k_1;
        }
        putchar('\n' as i32);
        n += 2 as std::os::raw::c_int;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    print_table(20 as std::os::raw::c_int as std::os::raw::c_uint, 21 as std::os::raw::c_int as std::os::raw::c_uint);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

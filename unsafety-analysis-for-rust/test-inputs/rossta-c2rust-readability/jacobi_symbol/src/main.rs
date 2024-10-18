#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn jacobi(
    mut a: libc::c_ulong,
    mut n: libc::c_ulong,
) -> libc::c_int {
    if a >= n {
        a = a.wrapping_rem(n);
    }
    let mut result: libc::c_int = 1 as libc::c_int;
    while a != 0 {
        while a & 1 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
        {
            a >>= 1 as libc::c_int;
            if n & 7 as libc::c_int as libc::c_ulong == 3 as libc::c_int as libc::c_ulong
                || n & 7 as libc::c_int as libc::c_ulong
                    == 5 as libc::c_int as libc::c_ulong
            {
                result = -result;
            }
        }
        a ^= n;
        n ^= a;
        a ^= n;
        if a & 3 as libc::c_int as libc::c_ulong == 3 as libc::c_int as libc::c_ulong
            && n & 3 as libc::c_int as libc::c_ulong == 3 as libc::c_int as libc::c_ulong
        {
            result = -result;
        }
        a = a.wrapping_rem(n);
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        return result;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_table(mut kmax: libc::c_uint, mut nmax: libc::c_uint) {
    printf(b"n\\k|\0" as *const u8 as *const libc::c_char);
    let mut k: libc::c_int = 0 as libc::c_int;
    while k as libc::c_uint <= kmax {
        printf(b"%'3u\0" as *const u8 as *const libc::c_char, k);
        k += 1;
        k;
    }
    printf(b"\n----\0" as *const u8 as *const libc::c_char);
    let mut k_0: libc::c_int = 0 as libc::c_int;
    while k_0 as libc::c_uint <= kmax {
        printf(b"---\0" as *const u8 as *const libc::c_char);
        k_0 += 1;
        k_0;
    }
    putchar('\n' as i32);
    let mut n: libc::c_int = 1 as libc::c_int;
    while n as libc::c_uint <= nmax {
        printf(b"%-2u |\0" as *const u8 as *const libc::c_char, n);
        let mut k_1: libc::c_int = 0 as libc::c_int;
        while k_1 as libc::c_uint <= kmax {
            printf(
                b"%'3d\0" as *const u8 as *const libc::c_char,
                jacobi(k_1 as libc::c_ulong, n as libc::c_ulong),
            );
            k_1 += 1;
            k_1;
        }
        putchar('\n' as i32);
        n += 2 as libc::c_int;
    }
}
unsafe fn main_0() -> libc::c_int {
    print_table(20 as libc::c_int as libc::c_uint, 21 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

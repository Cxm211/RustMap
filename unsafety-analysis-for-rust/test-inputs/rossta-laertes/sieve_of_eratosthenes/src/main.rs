#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eratosthenes<'a1>(
    mut n: std::os::raw::c_int,
    mut c: Option<&'a1 mut std::os::raw::c_int>,
) -> * mut std::os::raw::c_char {
    let mut sieve_0: *mut std::os::raw::c_char = core::ptr::null_mut();
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut m: std::os::raw::c_int = 0;
    if n < 2 as std::os::raw::c_int {
        return core::ptr::null_mut();
    }
    *borrow_mut(&mut c).unwrap() = n - 1 as std::os::raw::c_int;
    m = sqrt(n as std::os::raw::c_double) as std::os::raw::c_int;
    sieve_0 = calloc(
        (n + 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_char;
    *sieve_0.offset(0 as std::os::raw::c_int as isize) = 1 as std::os::raw::c_int as std::os::raw::c_char;
    *sieve_0.offset(1 as std::os::raw::c_int as isize) = 1 as std::os::raw::c_int as std::os::raw::c_char;
    i = 2 as std::os::raw::c_int;
    while i <= m {
        if *sieve_0.offset(i as isize) == 0 {
            j = i * i;
            while j <= n {
                if *sieve_0.offset(j as isize) == 0 {
                    *sieve_0.offset(j as isize) = 1 as std::os::raw::c_int as std::os::raw::c_char;
                    *borrow_mut(&mut c).unwrap() -= 1;
                    *borrow(& c).unwrap();
                }
                j += i;
            }
        }
        i += 1;
        i;
    }
    return sieve_0;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut array: *mut std::os::raw::c_int = core::ptr::null_mut();
    let mut n: std::os::raw::c_int = 10 as std::os::raw::c_int;
    array = malloc(
        ((n + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    sieve(array, n);
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sieve(mut a: *mut std::os::raw::c_int, mut n: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut j: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 2 as std::os::raw::c_int;
    while i <= n {
        *a.offset(i as isize) = 1 as std::os::raw::c_int;
        i += 1;
        i;
    }
    i = 2 as std::os::raw::c_int;
    while i <= n {
        printf(b"\ni:%d\0" as *const u8 as *const std::os::raw::c_char, i);
        if *a.offset(i as isize) == 1 as std::os::raw::c_int {
            j = i;
            while i * j <= n {
                printf(b"\nj:%d\0" as *const u8 as *const std::os::raw::c_char, j);
                printf(
                    b"\nBefore a[%d*%d]: %d\0" as *const u8 as *const std::os::raw::c_char,
                    i,
                    j,
                    *a.offset((i * j) as isize),
                );
                *a.offset((i * j) as isize) = 0 as std::os::raw::c_int;
                printf(
                    b"\nAfter a[%d*%d]: %d\0" as *const u8 as *const std::os::raw::c_char,
                    i,
                    j,
                    *a.offset((i * j) as isize),
                );
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    printf(
        b"\nPrimes numbers from 1 to %d are : \0" as *const u8 as *const std::os::raw::c_char,
        n,
    );
    i = 2 as std::os::raw::c_int;
    while i <= n {
        if *a.offset(i as isize) == 1 as std::os::raw::c_int {
            printf(b"%d, \0" as *const u8 as *const std::os::raw::c_char, i);
        }
        i += 1;
        i;
    }
    printf(b"\n\n\0" as *const u8 as *const std::os::raw::c_char);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

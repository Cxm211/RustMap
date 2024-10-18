#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
pub type bool_0 = std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn sieve<'a1>(
    mut limit: std::os::raw::c_int,
    mut primes: * mut std::os::raw::c_int,
    mut count: Option<&'a1 mut std::os::raw::c_int>,
) {
    let mut c: *mut bool_0 = calloc(
        (limit + 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
        std::mem::size_of::<bool_0>() as std::os::raw::c_ulong,
    ) as *mut bool_0;
    let mut i: std::os::raw::c_int = 0;
    let mut p: std::os::raw::c_int = 3 as std::os::raw::c_int;
    let mut p2: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0 as std::os::raw::c_int;
    p2 = p * p;
    while p2 <= limit {
        i = p2;
        while i <= limit {
            *c.offset(i as isize) = 1 as std::os::raw::c_int;
            i += 2 as std::os::raw::c_int * p;
        }
        loop {
            p += 2 as std::os::raw::c_int;
            if !(*c.offset(p as isize) != 0) {
                break;
            }
        }
        p2 = p * p;
    }
    i = 3 as std::os::raw::c_int;
    while i <= limit {
        if *c.offset(i as isize) == 0 {
            let fresh0 = n;
            n = n + 1;
            *primes.offset(fresh0 as isize) = i;
        }
        i += 2 as std::os::raw::c_int;
    }
    *borrow_mut(&mut count).unwrap() = n;
    free(c as *mut u8);
}
#[no_mangle]
pub extern "C" fn findPeriod(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut rr: std::os::raw::c_int = 0;
    let mut period: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 1 as std::os::raw::c_int;
    while i <= n + 1 as std::os::raw::c_int {
        r = 10 as std::os::raw::c_int * r % n;
        i += 1;
        i;
    }
    rr = r;
    loop {
        r = 10 as std::os::raw::c_int * r % n;
        period += 1;
        period;
        if !(r != rr) {
            break;
        }
    }
    return period;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut prime: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut index: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut primeCount: std::os::raw::c_int = 0;
    let mut longCount: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut numberCount: std::os::raw::c_int = 0;
    let mut primes: *mut std::os::raw::c_int = core::ptr::null_mut();
    let mut longPrimes: *mut std::os::raw::c_int = core::ptr::null_mut();
    let mut totals: *mut std::os::raw::c_int = core::ptr::null_mut();
    let mut numbers: [std::os::raw::c_int; 8] = [
        500 as std::os::raw::c_int,
        1000 as std::os::raw::c_int,
        2000 as std::os::raw::c_int,
        4000 as std::os::raw::c_int,
        8000 as std::os::raw::c_int,
        16000 as std::os::raw::c_int,
        32000 as std::os::raw::c_int,
        64000 as std::os::raw::c_int,
    ];
    primes = calloc(
        6500 as std::os::raw::c_int as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    numberCount = (std::mem::size_of::<[std::os::raw::c_int; 8]>() as std::os::raw::c_ulong)
        .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    totals = calloc(
        numberCount as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    sieve(64000 as std::os::raw::c_int, primes, Some(&mut primeCount));
    longPrimes = calloc(
        primeCount as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < primeCount {
        prime = *primes.offset(i as isize);
        if findPeriod(prime) == prime - 1 as std::os::raw::c_int {
            let fresh1 = longCount;
            longCount = longCount + 1;
            *longPrimes.offset(fresh1 as isize) = prime;
        }
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < longCount {
        if *longPrimes.offset(i as isize) > numbers[index as usize] {
            let fresh2 = index;
            index = index + 1;
            *totals.offset(fresh2 as isize) = count;
        }
        i += 1;
        i;
        count += 1;
        count;
    }
    *totals.offset((numberCount - 1 as std::os::raw::c_int) as isize) = count;
    printf(
        b"The long primes up to %d are:\n\0" as *const u8 as *const std::os::raw::c_char,
        numbers[0 as std::os::raw::c_int as usize],
    );
    printf(b"[\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < *totals.offset(0 as std::os::raw::c_int as isize) {
        printf(
            b"%d \0" as *const u8 as *const std::os::raw::c_char,
            *longPrimes.offset(i as isize),
        );
        i += 1;
        i;
    }
    printf(b"\x08]\n\0" as *const u8 as *const std::os::raw::c_char);
    printf(
        b"\nThe number of long primes up to:\n\0" as *const u8 as *const std::os::raw::c_char,
    );
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        printf(
            b"  %5d is %d\n\0" as *const u8 as *const std::os::raw::c_char,
            numbers[i as usize],
            *totals.offset(i as isize),
        );
        i += 1;
        i;
    }
    free(totals as *mut u8);
    free(longPrimes as *mut u8);
    free(primes as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type byte = std::os::raw::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Transition {
    pub a: std::os::raw::c_uchar,
    pub b: std::os::raw::c_uchar,
    pub c: std::os::raw::c_uint,
}
impl std::default::Default for Transition {
    fn default() -> Self {
        Transition {
        a: std::os::raw::c_uchar::default(),
        b: std::os::raw::c_uchar::default(),
        c: std::os::raw::c_uint::default()
        }
    }
}

#[no_mangle]
pub static mut transitions: [Transition; 100] = [Transition { a: 0, b: 0, c: 0 }; 100];
#[no_mangle]
pub unsafe extern "C" fn init() {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        j = 0 as std::os::raw::c_int;
        while j < 10 as std::os::raw::c_int {
            let mut idx: std::os::raw::c_int = i * 10 as std::os::raw::c_int + j;
            transitions[idx as usize].a = i as byte;
            transitions[idx as usize].b = j as byte;
            transitions[idx as usize].c = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn record(mut prev: std::os::raw::c_int, mut curr: std::os::raw::c_int) {
    let mut pd: byte = (prev % 10 as std::os::raw::c_int) as byte;
    let mut cd: byte = (curr % 10 as std::os::raw::c_int) as byte;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 100 as std::os::raw::c_int {
        let mut z: std::os::raw::c_int = 0 as std::os::raw::c_int;
        if transitions[i as usize].a as std::os::raw::c_int == pd as std::os::raw::c_int {
            let mut t: std::os::raw::c_int = 0 as std::os::raw::c_int;
            if transitions[i as usize].b as std::os::raw::c_int == cd as std::os::raw::c_int {
                transitions[i as usize].c = (transitions[i as usize].c).wrapping_add(1);
                transitions[i as usize].c;
                break;
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn printTransitions(
    mut limit: std::os::raw::c_int,
    mut last_prime: std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    printf(
        b"%d primes, last prime considered: %d\n\0" as *const u8 as *const std::os::raw::c_char,
        limit,
        last_prime,
    );
    i = 0 as std::os::raw::c_int;
    while i < 100 as std::os::raw::c_int {
        if transitions[i as usize].c > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            printf(
                b"%d->%d  count: %5d  frequency: %.2f\n\0" as *const u8
                    as *const std::os::raw::c_char,
                transitions[i as usize].a as std::os::raw::c_int,
                transitions[i as usize].b as std::os::raw::c_int,
                transitions[i as usize].c,
                100.0f64 * transitions[i as usize].c as std::os::raw::c_double
                    / limit as std::os::raw::c_double,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn isPrime(mut n: std::os::raw::c_int) -> bool {
    let mut s: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut a1: std::os::raw::c_int = 0;
    let mut a2: std::os::raw::c_int = 0;
    if n % 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 2 as std::os::raw::c_int;
    }
    if n % 3 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 3 as std::os::raw::c_int;
    }
    if n % 5 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 5 as std::os::raw::c_int;
    }
    if n % 7 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 7 as std::os::raw::c_int;
    }
    if n % 11 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 11 as std::os::raw::c_int;
    }
    if n % 13 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 13 as std::os::raw::c_int;
    }
    if n % 17 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 17 as std::os::raw::c_int;
    }
    if n % 19 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return n == 19 as std::os::raw::c_int;
    }
    t = 23 as std::os::raw::c_int;
    a1 = 96 as std::os::raw::c_int;
    a2 = 216 as std::os::raw::c_int;
    s = t * t;
    while s <= n {
        if n % t == 0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int != 0;
        }
        s += a1;
        t += 2 as std::os::raw::c_int;
        a1 += 24 as std::os::raw::c_int;
        if t * t == s {} else {
            __assert_fail(
                b"t * t == s\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                77 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 19],
                    &[std::os::raw::c_char; 19],
                >(b"_Bool isPrime(int)\0"))
                    .as_ptr(),
            );
        }
        'c_923: {
            if t * t == s {} else {
                __assert_fail(
                    b"t * t == s\0" as *const u8 as *const std::os::raw::c_char,
                    b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                    77 as std::os::raw::c_int as std::os::raw::c_uint,
                    (*std::mem::transmute::<
                        &[u8; 19],
                        &[std::os::raw::c_char; 19],
                    >(b"_Bool isPrime(int)\0"))
                        .as_ptr(),
                );
            }
        };
        if s <= n {
            if n % t == 0 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int != 0;
            }
            s += a2;
            t += 4 as std::os::raw::c_int;
            a2 += 48 as std::os::raw::c_int;
            if t * t == s {} else {
                __assert_fail(
                    b"t * t == s\0" as *const u8 as *const std::os::raw::c_char,
                    b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                    86 as std::os::raw::c_int as std::os::raw::c_uint,
                    (*std::mem::transmute::<
                        &[u8; 19],
                        &[std::os::raw::c_char; 19],
                    >(b"_Bool isPrime(int)\0"))
                        .as_ptr(),
                );
            }
            'c_846: {
                if t * t == s {} else {
                    __assert_fail(
                        b"t * t == s\0" as *const u8 as *const std::os::raw::c_char,
                        b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                        86 as std::os::raw::c_int as std::os::raw::c_uint,
                        (*std::mem::transmute::<
                            &[u8; 19],
                            &[std::os::raw::c_char; 19],
                        >(b"_Bool isPrime(int)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
    }
    return 1 as std::os::raw::c_int != 0;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut last_prime: std::os::raw::c_int = 3 as std::os::raw::c_int;
    let mut n: std::os::raw::c_int = 5 as std::os::raw::c_int;
    let mut count: std::os::raw::c_int = 2 as std::os::raw::c_int;
    init();
    record(2 as std::os::raw::c_int, 3 as std::os::raw::c_int);
    while count < 1000000 as std::os::raw::c_int {
        if isPrime(n) {
            record(last_prime, n);
            last_prime = n;
            count += 1;
            count;
        }
        n += 2 as std::os::raw::c_int;
        if count < 1000000 as std::os::raw::c_int {
            if isPrime(n) {
                record(last_prime, n);
                last_prime = n;
                count += 1;
                count;
            }
            n += 4 as std::os::raw::c_int;
        }
    }
    printTransitions(1000000 as std::os::raw::c_int, last_prime);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

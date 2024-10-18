#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type bool_0 = std::os::raw::c_int;
#[no_mangle]
pub extern "C" fn is_prime(mut n: std::os::raw::c_int) -> bool_0 {
    let mut d: std::os::raw::c_int = 5 as std::os::raw::c_int;
    if n < 2 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if n % 2 as std::os::raw::c_int == 0 {
        return (n == 2 as std::os::raw::c_int) as std::os::raw::c_int;
    }
    if n % 3 as std::os::raw::c_int == 0 {
        return (n == 3 as std::os::raw::c_int) as std::os::raw::c_int;
    }
    while d * d <= n {
        if n % d == 0 {
            return 0 as std::os::raw::c_int;
        }
        d += 2 as std::os::raw::c_int;
        if n % d == 0 {
            return 0 as std::os::raw::c_int;
        }
        d += 4 as std::os::raw::c_int;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn count_prime_factors(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut f: std::os::raw::c_int = 2 as std::os::raw::c_int;
    if n == 1 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if is_prime(n) != 0 {
        return 1 as std::os::raw::c_int;
    }
    loop {
        if n % f == 0 {
            count += 1;
            count;
            n /= f;
            if n == 1 as std::os::raw::c_int {
                return count;
            }
            if is_prime(n) != 0 {
                f = n;
            }
        } else if f >= 3 as std::os::raw::c_int {
            f += 2 as std::os::raw::c_int;
        } else {
            f = 3 as std::os::raw::c_int;
        }
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    printf(
        b"The attractive numbers up to and including %d are:\n\0" as *const u8
            as *const std::os::raw::c_char,
        120 as std::os::raw::c_int,
    );
    i = 1 as std::os::raw::c_int;
    while i <= 120 as std::os::raw::c_int {
        n = count_prime_factors(i);
        if is_prime(n) != 0 {
            printf(b"%4d\0" as *const u8 as *const std::os::raw::c_char, i);
            count += 1;
            if count % 20 as std::os::raw::c_int == 0 {
                printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
            }
        }
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

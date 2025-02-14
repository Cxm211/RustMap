#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn proper_divisors(
    n: std::os::raw::c_int,
    mut print_flag: bool,
) -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while i < n {
        if n % i == 0 as std::os::raw::c_int {
            count += 1;
            count;
            if print_flag {
                printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, i);
            }
        }
        i += 1;
        i;
    }
    if print_flag {
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn countProperDivisors(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut prod: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while n % 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        count += 1;
        count;
        n /= 2 as std::os::raw::c_int;
    }
    prod *= 1 as std::os::raw::c_int + count;
    i = 3 as std::os::raw::c_int;
    while i * i <= n {
        count = 0 as std::os::raw::c_int;
        while n % i == 0 as std::os::raw::c_int {
            count += 1;
            count;
            n /= i;
        }
        prod *= 1 as std::os::raw::c_int + count;
        i += 2 as std::os::raw::c_int;
    }
    if n > 2 as std::os::raw::c_int {
        prod *= 2 as std::os::raw::c_int;
    }
    return prod - 1 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while i <= 10 as std::os::raw::c_int {
        printf(b"%d: \0" as *const u8 as *const std::os::raw::c_char, i);
        proper_divisors(i, 1 as std::os::raw::c_int != 0);
        i += 1;
        i;
    }
    let mut max: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut max_i: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut i_0: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while i_0 <= 20000 as std::os::raw::c_int {
        let mut v: std::os::raw::c_int = countProperDivisors(i_0);
        if v >= max {
            max = v;
            max_i = i_0;
        }
        i_0 += 1;
        i_0;
    }
    printf(b"%d with %d divisors\n\0" as *const u8 as *const std::os::raw::c_char, max_i, max);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

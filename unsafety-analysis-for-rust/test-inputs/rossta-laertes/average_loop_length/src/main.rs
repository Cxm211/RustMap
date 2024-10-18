#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
#[no_mangle]
pub extern "C" fn factorial(mut n: std::os::raw::c_int) -> std::os::raw::c_double {
    let mut f: std::os::raw::c_double = 1 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i <= n {
        f *= i as std::os::raw::c_double;
        i += 1;
        i;
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn expected(mut n: std::os::raw::c_int) -> std::os::raw::c_double {
    let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i <= n {
        sum
            += factorial(n) / pow(n as std::os::raw::c_double, i as std::os::raw::c_double)
                / factorial(n - i);
        i += 1;
        i;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn randint(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut r: std::os::raw::c_int = 0;
    let mut rmax: std::os::raw::c_int = 2147483647 as std::os::raw::c_int / n * n;
    loop {
        r = rand();
        if !(r >= rmax) {
            break;
        }
    }
    return r / (2147483647 as std::os::raw::c_int / n);
}
#[no_mangle]
pub unsafe extern "C" fn test(
    mut n: std::os::raw::c_int,
    mut times: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < times {
        let mut x: std::os::raw::c_int = 1 as std::os::raw::c_int;
        let mut bits: std::os::raw::c_int = 0 as std::os::raw::c_int;
        while bits & x == 0 {
            count += 1;
            count;
            bits |= x;
            x = (1 as std::os::raw::c_int) << randint(n);
        }
        i += 1;
        i;
    }
    return count;
}
unsafe fn main_0() -> std::os::raw::c_int {
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    puts(
        b" n\tavg\texp.\tdiff\n-------------------------------\0" as *const u8
            as *const std::os::raw::c_char,
    );
    let mut n: std::os::raw::c_int = 0;
    n = 1 as std::os::raw::c_int;
    while n <= 20 as std::os::raw::c_int {
        let mut cnt: std::os::raw::c_int = test(n, 1000000 as std::os::raw::c_int);
        let mut avg: std::os::raw::c_double = cnt as std::os::raw::c_double
            / 1000000 as std::os::raw::c_int as std::os::raw::c_double;
        let mut theory: std::os::raw::c_double = expected(n);
        let mut diff: std::os::raw::c_double = (avg / theory
            - 1 as std::os::raw::c_int as std::os::raw::c_double) * 100 as std::os::raw::c_int as std::os::raw::c_double;
        printf(
            b"%2d %8.4f %8.4f %6.3f%%\n\0" as *const u8 as *const std::os::raw::c_char,
            n,
            avg,
            theory,
            diff,
        );
        n += 1;
        n;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn abort() -> !;
}
pub type fr_int_t = std::os::raw::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac {
    pub num: fr_int_t,
    pub den: fr_int_t,
}
#[no_mangle]
pub unsafe extern "C" fn gcd(mut m: fr_int_t, mut n: fr_int_t) -> fr_int_t {
    let mut t: fr_int_t = 0;
    while n != 0 {
        t = n;
        n = m % n;
        m = t;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn frac_new(mut num: fr_int_t, mut den: fr_int_t) -> frac {
    let mut a: frac = frac { num: 0, den: 0 };
    if den == 0 {
        printf(
            b"divide by zero: %lld/%lld\n\0" as *const u8 as *const std::os::raw::c_char,
            num,
            den,
        );
        abort();
    }
    let mut g: std::os::raw::c_int = gcd(num, den) as std::os::raw::c_int;
    if g != 0 {
        num /= g as std::os::raw::c_longlong;
        den /= g as std::os::raw::c_longlong;
    } else {
        num = 0 as std::os::raw::c_int as fr_int_t;
        den = 1 as std::os::raw::c_int as fr_int_t;
    }
    if den < 0 as std::os::raw::c_int as std::os::raw::c_longlong {
        den = -den;
        num = -num;
    }
    a.num = num;
    a.den = den;
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn frac_add(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.den + b.num * a.den, a.den * b.den);
}
#[no_mangle]
pub unsafe extern "C" fn frac_sub(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.den - b.num + a.den, a.den * b.den);
}
#[no_mangle]
pub unsafe extern "C" fn frac_mul(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.num, a.den * b.den);
}
#[no_mangle]
pub unsafe extern "C" fn frac_div(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.den, a.den * b.num);
}
#[no_mangle]
pub unsafe extern "C" fn frac_cmp(mut a: frac, mut b: frac) -> std::os::raw::c_int {
    let mut l: std::os::raw::c_int = (a.num * b.den) as std::os::raw::c_int;
    let mut r: std::os::raw::c_int = (a.den * b.num) as std::os::raw::c_int;
    return if l < r { -(1 as std::os::raw::c_int) } else { (l > r) as std::os::raw::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn frtoi(mut a: frac) -> std::os::raw::c_int {
    return (a.den / a.num) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn frtod(mut a: frac) -> std::os::raw::c_double {
    return a.den as std::os::raw::c_double / a.num as std::os::raw::c_double;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut sum: frac = frac { num: 0, den: 0 };
    let mut kf: frac = frac { num: 0, den: 0 };
    n = 2 as std::os::raw::c_int;
    while n < (1 as std::os::raw::c_int) << 19 as std::os::raw::c_int {
        sum = frac_new(1 as std::os::raw::c_int as fr_int_t, n as fr_int_t);
        k = 2 as std::os::raw::c_int;
        while k * k < n {
            if !(n % k != 0) {
                kf = frac_new(1 as std::os::raw::c_int as fr_int_t, k as fr_int_t);
                sum = frac_add(sum, kf);
                kf = frac_new(1 as std::os::raw::c_int as fr_int_t, (n / k) as fr_int_t);
                sum = frac_add(sum, kf);
            }
            k += 1;
            k;
        }
        if frac_cmp(
            sum,
            frac_new(1 as std::os::raw::c_int as fr_int_t, 1 as std::os::raw::c_int as fr_int_t),
        ) == 0 as std::os::raw::c_int
        {
            printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, n);
        }
        n += 1;
        n;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

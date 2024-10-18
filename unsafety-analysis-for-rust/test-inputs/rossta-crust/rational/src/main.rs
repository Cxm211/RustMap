#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn abort() -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac {
    pub num: i64,
    pub den: i64,
}
#[no_mangle]
pub extern "C" fn gcd(mut m: i64, mut n: i64) -> i64 {
    let mut t: i64 = 0;
    while n != 0 {
        t = n;
        n = m % n;
        m = t;
    }
    return m;
}

#[no_mangle]
pub extern "C" fn frac_new(mut num: i64, mut den: i64) -> frac {
    let mut a: frac = frac { num: 0, den: 0 };
    unsafe {
        if den == 0 {
            print!("divide by zero: {}/{}\n", num, den);
            abort();
        }
    }
    let mut g: i32 = gcd(num, den) as i32;
    if g != 0 {
        num /= g as i64;
        den /= g as i64;
    } else {
        num = 0;
        den = 1;
    }
    if den < 0 {
        den = -den;
        num = -num;
    }
    a.num = num;
    a.den = den;
    return a;
}

#[no_mangle]
pub extern "C" fn frac_add(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.den + b.num * a.den, a.den * b.den);
}

#[no_mangle]
pub extern "C" fn frac_sub(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.den - b.num + a.den, a.den * b.den);
}

#[no_mangle]
pub extern "C" fn frac_mul(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.num, a.den * b.den);
}

#[no_mangle]
pub extern "C" fn frac_div(mut a: frac, mut b: frac) -> frac {
    return frac_new(a.num * b.den, a.den * b.num);
}

#[no_mangle]
pub extern "C" fn frac_cmp(mut a: frac, mut b: frac) -> i32 {
    let mut l: i32 = (a.num * b.den) as i32;
    let mut r: i32 = (a.den * b.num) as i32;
    return if l < r { -1 } else { (l > r) as i32 };
}

#[no_mangle]
pub extern "C" fn frtoi(mut a: frac) -> i32 {
    return (a.den / a.num) as i32;
}

#[no_mangle]
pub extern "C" fn frtod(mut a: frac) -> f64 {
    return a.den as f64 / a.num as f64;
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut sum: frac = frac { num: 0, den: 0 };
    let mut kf: frac = frac { num: 0, den: 0 };
    n = 2;
    while n < 1 << 19 {
        sum = frac_new(1, n as i64);
        k = 2;
        while k * k < n {
            if !(n % k != 0) {
                kf = frac_new(1, k as i64);
                sum = frac_add(sum, kf);
                kf = frac_new(1, (n / k) as i64);
                sum = frac_add(sum, kf);
            }
            k += 1;
            k;
        }
        if frac_cmp(sum, frac_new(1, 1)) == 0 {
            print!("{}\n", n);
        }
        n += 1;
        n;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

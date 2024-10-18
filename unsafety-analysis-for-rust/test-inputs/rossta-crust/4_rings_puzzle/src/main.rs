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
extern "C" {}
#[no_mangle]
pub static mut a: i32 = 0;
#[no_mangle]
pub static mut b: i32 = 0;
#[no_mangle]
pub static mut c: i32 = 0;
#[no_mangle]
pub static mut d: i32 = 0;
#[no_mangle]
pub static mut e: i32 = 0;
#[no_mangle]
pub static mut f: i32 = 0;
#[no_mangle]
pub static mut g: i32 = 0;
#[no_mangle]
pub static mut lo: i32 = 0;
#[no_mangle]
pub static mut hi: i32 = 0;
#[no_mangle]
pub static mut unique: i32 = 0;
#[no_mangle]
pub static mut show: i32 = 0;
#[no_mangle]
pub static mut solutions: i32 = 0;
#[no_mangle]
pub extern "C" fn bf() {
    unsafe {
        f = lo;
        while f <= hi {
            if unique == 0 || f != a && f != c && f != d && f != g && f != e {
                b = e + f - c;
                if b >= lo
                    && b <= hi
                    && (unique == 0 || b != a && b != c && b != d && b != g && b != e && b != f)
                {
                    solutions += 1;
                    solutions;
                    if show != 0 {
                        print!("{} {} {} {} {} {} {}\n", a, b, c, d, e, f, g);
                    }
                }
            }
            f += 1;
            f;
        }
    }
}

#[no_mangle]
pub extern "C" fn ge() {
    unsafe {
        e = lo;
        while e <= hi {
            if unique == 0 || e != a && e != c && e != d {
                g = d + e;
                if g >= lo && g <= hi && (unique == 0 || g != a && g != c && g != d && g != e) {
                    bf();
                }
            }
            e += 1;
            e;
        }
    }
}

#[no_mangle]
pub extern "C" fn acd() {
    unsafe {
        c = lo;
        while c <= hi {
            d = lo;
            while d <= hi {
                if unique == 0 || c != d {
                    a = c + d;
                    if a >= lo && a <= hi && (unique == 0 || c != 0 && d != 0) {
                        ge();
                    }
                }
                d += 1;
                d;
            }
            c += 1;
            c;
        }
    }
}

#[no_mangle]
pub extern "C" fn foursquares(mut plo: i32, mut phi: i32, mut punique: i32, mut pshow: i32) {
    unsafe {
        lo = plo;
        hi = phi;
        unique = punique;
        show = pshow;
        solutions = 0;
    }
    print!("\n");
    acd();
    unsafe {
        if unique != 0 {
            print!("\n{} unique solutions in {} to {}\n", solutions, lo, hi);
        } else {
            print!("\n{} non-unique solutions in {} to {}\n", solutions, lo, hi);
        };
    }
}

fn main_0() -> i32 {
    foursquares(1, 7, 1, 1);
    foursquares(3, 9, 1, 1);
    foursquares(0, 9, 0, 0);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

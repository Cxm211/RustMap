#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub static mut a: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut b: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut c: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut d: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut e: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut f: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut g: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut lo: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut hi: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut unique: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut show: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut solutions: std::os::raw::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn bf() {
    f = lo;
    while f <= hi {
        if unique == 0 || f != a && f != c && f != d && f != g && f != e {
            b = e + f - c;
            if b >= lo && b <= hi
                && (unique == 0
                    || b != a && b != c && b != d && b != g && b != e && b != f)
            {
                solutions += 1;
                solutions;
                if show != 0 {
                    printf(
                        b"%d %d %d %d %d %d %d\n\0" as *const u8 as *const std::os::raw::c_char,
                        a,
                        b,
                        c,
                        d,
                        e,
                        f,
                        g,
                    );
                }
            }
        }
        f += 1;
        f;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ge() {
    e = lo;
    while e <= hi {
        if unique == 0 || e != a && e != c && e != d {
            g = d + e;
            if g >= lo && g <= hi
                && (unique == 0 || g != a && g != c && g != d && g != e)
            {
                bf();
            }
        }
        e += 1;
        e;
    }
}
#[no_mangle]
pub unsafe extern "C" fn acd() {
    c = lo;
    while c <= hi {
        d = lo;
        while d <= hi {
            if unique == 0 || c != d {
                a = c + d;
                if a >= lo && a <= hi
                    && (unique == 0 || c != 0 as std::os::raw::c_int && d != 0 as std::os::raw::c_int)
                {
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
#[no_mangle]
pub unsafe extern "C" fn foursquares(
    mut plo: std::os::raw::c_int,
    mut phi: std::os::raw::c_int,
    mut punique: std::os::raw::c_int,
    mut pshow: std::os::raw::c_int,
) {
    lo = plo;
    hi = phi;
    unique = punique;
    show = pshow;
    solutions = 0 as std::os::raw::c_int;
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    acd();
    if unique != 0 {
        printf(
            b"\n%d unique solutions in %d to %d\n\0" as *const u8 as *const std::os::raw::c_char,
            solutions,
            lo,
            hi,
        );
    } else {
        printf(
            b"\n%d non-unique solutions in %d to %d\n\0" as *const u8
                as *const std::os::raw::c_char,
            solutions,
            lo,
            hi,
        );
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    foursquares(1 as std::os::raw::c_int, 7 as std::os::raw::c_int, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int);
    foursquares(3 as std::os::raw::c_int, 9 as std::os::raw::c_int, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int);
    foursquares(0 as std::os::raw::c_int, 9 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

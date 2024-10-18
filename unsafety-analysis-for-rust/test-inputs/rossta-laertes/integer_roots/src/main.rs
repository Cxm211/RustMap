#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[macro_use]
extern crate num_traits;
// //use ::c2rust_out::*;
use num_traits::ToPrimitive;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn powl(_: f128::f128, _: f128::f128) -> f128::f128;
}
pub type ulong = std::os::raw::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn root(mut base: ulong, mut n: ulong) -> ulong {
    let mut n1: ulong = 0;
    let mut n2: ulong = 0;
    let mut n3: ulong = 0;
    let mut c: ulong = 0;
    let mut d: ulong = 0;
    let mut e: ulong = 0;
    if base < 2 as std::os::raw::c_int as std::os::raw::c_ulonglong {
        return base;
    }
    if n == 0 as std::os::raw::c_int as std::os::raw::c_ulonglong {
        return 1 as std::os::raw::c_int as ulong;
    }
    n1 = n.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong);
    n2 = n;
    n3 = n1;
    c = 1 as std::os::raw::c_int as ulong;
    d = n3.wrapping_add(base).wrapping_div(n2);
    e = n3
        .wrapping_mul(d)
        .wrapping_add(
            base
                .wrapping_div(
                    (powl(f128::f128::new(d), f128::f128::new(n1))).to_u64().unwrap(),
                ),
        )
        .wrapping_div(n2);
    while c != d && c != e {
        c = d;
        d = e;
        e = n3
            .wrapping_mul(e)
            .wrapping_add(
                base
                    .wrapping_div(
                        (powl(f128::f128::new(e), f128::f128::new(n1))).to_u64().unwrap(),
                    ),
            )
            .wrapping_div(n2);
    }
    if d < e {
        return d;
    }
    return e;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut b: ulong = 2e18f64 as ulong;
    printf(
        b"3rd root of 8 = %lld\n\0" as *const u8 as *const std::os::raw::c_char,
        root(8 as std::os::raw::c_int as ulong, 3 as std::os::raw::c_int as ulong),
    );
    printf(
        b"3rd root of 9 = %lld\n\0" as *const u8 as *const std::os::raw::c_char,
        root(9 as std::os::raw::c_int as ulong, 3 as std::os::raw::c_int as ulong),
    );
    printf(
        b"2nd root of %lld = %lld\n\0" as *const u8 as *const std::os::raw::c_char,
        b,
        root(b, 2 as std::os::raw::c_int as ulong),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

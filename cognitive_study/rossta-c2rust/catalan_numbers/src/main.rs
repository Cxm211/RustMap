#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
pub type ull = std::os::raw::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn binomial(mut m: ull, mut n: ull) -> ull {
    let mut r: ull = 1 as std::os::raw::c_int as ull;
    let mut d: ull = m.wrapping_sub(n);
    if d > n {
        n = d;
        d = m.wrapping_sub(n);
    }
    while m > n {
        let fresh0 = m;
        m = m.wrapping_sub(1);
        r = (r as std::os::raw::c_ulonglong).wrapping_mul(fresh0) as ull as ull;
        while d > 1 as std::os::raw::c_int as std::os::raw::c_ulonglong && r.wrapping_rem(d) == 0 {
            let fresh1 = d;
            d = d.wrapping_sub(1);
            r = (r as std::os::raw::c_ulonglong).wrapping_div(fresh1) as ull as ull;
        }
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn catalan1(mut n: std::os::raw::c_int) -> ull {
    return (binomial((2 as std::os::raw::c_int * n) as ull, n as ull))
        .wrapping_div((1 as std::os::raw::c_int + n) as std::os::raw::c_ulonglong);
}
#[no_mangle]
pub unsafe extern "C" fn catalan2(mut n: std::os::raw::c_int) -> ull {
    let mut i: std::os::raw::c_int = 0;
    let mut r: ull = (n == 0) as std::os::raw::c_int as ull;
    i = 0 as std::os::raw::c_int;
    while i < n {
        r = (r as std::os::raw::c_ulonglong)
            .wrapping_add((catalan2(i)).wrapping_mul(catalan2(n - 1 as std::os::raw::c_int - i)))
            as ull as ull;
        i += 1;
        i;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn catalan3(mut n: std::os::raw::c_int) -> ull {
    return if n != 0 {
        ((2 as std::os::raw::c_int * (2 as std::os::raw::c_int * n - 1 as std::os::raw::c_int))
            as std::os::raw::c_ulonglong)
            .wrapping_mul(catalan3(n - 1 as std::os::raw::c_int))
            .wrapping_div((1 as std::os::raw::c_int + n) as std::os::raw::c_ulonglong)
    } else {
        1 as std::os::raw::c_int as std::os::raw::c_ulonglong
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    puts(b"\tdirect\tsumming\tfrac\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        printf(
            b"%d\t%llu\t%llu\t%llu\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            catalan1(i),
            catalan2(i),
            catalan3(i),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

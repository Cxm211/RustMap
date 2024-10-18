#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
 extern "C" fn gcd_ui(
    mut x: std::os::raw::c_ulong,
    mut y: std::os::raw::c_ulong,
) -> std::os::raw::c_ulong {
    let mut t: std::os::raw::c_ulong = 0;
    if y < x {
        t = x;
        x = y;
        y = t;
    }
    while y > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        t = y;
        y = x.wrapping_rem(y);
        x = t;
    }
    return x;
}
#[no_mangle]
pub extern "C" fn binomial(
    mut n: std::os::raw::c_ulong,
    mut k: std::os::raw::c_ulong,
) -> std::os::raw::c_ulong {
    let mut d: std::os::raw::c_ulong = 0;
    let mut g: std::os::raw::c_ulong = 0;
    let mut r: std::os::raw::c_ulong = 1 as std::os::raw::c_int as std::os::raw::c_ulong;
    if k == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 1 as std::os::raw::c_int as std::os::raw::c_ulong;
    }
    if k == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return n;
    }
    if k >= n {
        return (k == n) as std::os::raw::c_int as std::os::raw::c_ulong;
    }
    if k > n.wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) {
        k = n.wrapping_sub(k);
    }
    d = 1 as std::os::raw::c_int as std::os::raw::c_ulong;
    while d <= k {
        if r
            >= (9223372036854775807 as std::os::raw::c_long as std::os::raw::c_ulong)
                .wrapping_mul(2 as std::os::raw::c_ulong)
                .wrapping_add(1 as std::os::raw::c_ulong)
                .wrapping_div(n)
        {
            let mut nr: std::os::raw::c_ulong = 0;
            let mut dr: std::os::raw::c_ulong = 0;
            g = gcd_ui(n, d);
            nr = n.wrapping_div(g);
            dr = d.wrapping_div(g);
            g = gcd_ui(r, dr);
            r = r.wrapping_div(g);
            dr = dr.wrapping_div(g);
            if r
                >= (9223372036854775807 as std::os::raw::c_long as std::os::raw::c_ulong)
                    .wrapping_mul(2 as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_ulong)
                    .wrapping_div(nr)
            {
                return 0 as std::os::raw::c_int as std::os::raw::c_ulong;
            }
            r = r.wrapping_mul(nr);
            r = r.wrapping_div(dr);
            n = n.wrapping_sub(1);
            n;
        } else {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            r = r.wrapping_mul(fresh0);
            r = r.wrapping_div(d);
        }
        d = d.wrapping_add(1);
        d;
    }
    return r;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"%lu\n\0" as *const u8 as *const std::os::raw::c_char,
        binomial(5 as std::os::raw::c_int as std::os::raw::c_ulong, 3 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    printf(
        b"%lu\n\0" as *const u8 as *const std::os::raw::c_char,
        binomial(40 as std::os::raw::c_int as std::os::raw::c_ulong, 19 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    printf(
        b"%lu\n\0" as *const u8 as *const std::os::raw::c_char,
        binomial(67 as std::os::raw::c_int as std::os::raw::c_ulong, 31 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

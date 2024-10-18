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
extern "C" fn gcd_ui(mut x: u64, mut y: u64) -> u64 {
    let mut t: u64 = 0;
    if y < x {
        t = x;
        x = y;
        y = t;
    }
    while y > 0 {
        t = y;
        y = x.wrapping_rem(y);
        x = t;
    }
    return x;
}

#[no_mangle]
pub extern "C" fn binomial(mut n: u64, mut k: u64) -> u64 {
    let mut d: u64 = 0;
    let mut g: u64 = 0;
    let mut r: u64 = 1;
    if k == 0 {
        return 1;
    }
    if k == 1 {
        return n;
    }
    if k >= n {
        return (k == n) as u64;
    }
    if k > n.wrapping_div(2) {
        k = n.wrapping_sub(k);
    }
    d = 1;
    while d <= k {
        if r >= 9223372036854775807u64
            .wrapping_mul(2)
            .wrapping_add(1)
            .wrapping_div(n)
        {
            let mut nr: u64 = 0;
            let mut dr: u64 = 0;
            g = gcd_ui(n, d);
            nr = n.wrapping_div(g);
            dr = d.wrapping_div(g);
            g = gcd_ui(r, dr);
            r = r.wrapping_div(g);
            dr = dr.wrapping_div(g);
            if r >= 9223372036854775807u64
                .wrapping_mul(2)
                .wrapping_add(1)
                .wrapping_div(nr)
            {
                return 0;
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

fn main_0() -> i32 {
    print!("{}\n", binomial(5, 3));
    print!("{}\n", binomial(40, 19));
    print!("{}\n", binomial(67, 31));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
    fn puts(__s: *const i8) -> i32;
}
#[no_mangle]
pub extern "C" fn binomial(mut m: u64, mut n: u64) -> u64 {
    let mut r: u64 = 1;
    let mut d: u64 = m.wrapping_sub(n);
    if d > n {
        n = d;
        d = m.wrapping_sub(n);
    }
    while m > n {
        let fresh0 = m;
        m = m.wrapping_sub(1);
        r = (r).wrapping_mul(fresh0) as u64;
        while d > 1 && r.wrapping_rem(d) == 0 {
            let fresh1 = d;
            d = d.wrapping_sub(1);
            r = (r).wrapping_div(fresh1) as u64;
        }
    }
    return r;
}

#[no_mangle]
pub extern "C" fn catalan1(mut n: i32) -> u64 {
    return (binomial((2 * n) as u64, n as u64)).wrapping_div((1 + n) as u64);
}

#[no_mangle]
pub extern "C" fn catalan2(mut n: i32) -> u64 {
    let mut i: i32 = 0;
    let mut r: u64 = (n == 0) as u64;
    i = 0;
    while i < n {
        r = (r).wrapping_add((catalan2(i)).wrapping_mul(catalan2(n - 1 - i))) as u64;
        i += 1;
        i;
    }
    return r;
}

#[no_mangle]
pub extern "C" fn catalan3(mut n: i32) -> u64 {
    return if n != 0 {
        ((2 * (2 * n - 1i32)) as u64)
            .wrapping_mul(catalan3(n - 1))
            .wrapping_div((1 + n) as u64)
    } else {
        1
    };
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    unsafe {
        puts(b"\tdirect\tsumming\tfrac\0" as *const u8 as *const i8);
    }
    i = 0;
    while i < 16 {
        print!("{}	{}	{}	{}\n", i, catalan1(i), catalan2(i), catalan3(i));
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

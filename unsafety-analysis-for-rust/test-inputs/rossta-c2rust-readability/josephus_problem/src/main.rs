#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
pub type xint = std::os::raw::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn jos(
    mut n: std::os::raw::c_int,
    mut k: std::os::raw::c_int,
    mut m: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut a: std::os::raw::c_int = 0;
    a = m + 1 as std::os::raw::c_int;
    while a <= n {
        m = (m + k) % a;
        a += 1;
        a;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn jos_large(mut n: xint, mut k: xint, mut m: xint) -> xint {
    if k <= 1 as std::os::raw::c_int as std::os::raw::c_ulonglong {
        return n.wrapping_sub(m).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong);
    }
    let mut a: xint = m;
    while a < n {
        let mut q: xint = a
            .wrapping_sub(m)
            .wrapping_add(k)
            .wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_ulonglong)
            .wrapping_div(k.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong));
        if a.wrapping_add(q) > n {
            q = n.wrapping_sub(a);
        } else if q == 0 {
            q = 1 as std::os::raw::c_int as xint;
        }
        a = (a as std::os::raw::c_ulonglong).wrapping_add(q) as xint as xint;
        m = m.wrapping_add(q.wrapping_mul(k)).wrapping_rem(a);
    }
    return m;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: xint = 0;
    let mut k: xint = 0;
    let mut i: xint = 0;
    n = 41 as std::os::raw::c_int as xint;
    k = 3 as std::os::raw::c_int as xint;
    printf(
        b"n = %llu, k = %llu, final survivor: %d\n\0" as *const u8
            as *const std::os::raw::c_char,
        n,
        k,
        jos(n as std::os::raw::c_int, k as std::os::raw::c_int, 0 as std::os::raw::c_int),
    );
    n = 9876543210987654321 as std::os::raw::c_ulonglong;
    k = 12031 as std::os::raw::c_int as xint;
    printf(
        b"n = %llu, k = %llu, three survivors:\0" as *const u8 as *const std::os::raw::c_char,
        n,
        k,
    );
    i = 3 as std::os::raw::c_int as xint;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        printf(b" %llu\0" as *const u8 as *const std::os::raw::c_char, jos_large(n, k, i));
    }
    putchar('\n' as i32);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn is_prime(mut n: std::os::raw::c_uint) -> std::os::raw::c_int {
    if n <= 3 as std::os::raw::c_int as std::os::raw::c_uint {
        return (n > 1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int
    } else if n.wrapping_rem(2 as std::os::raw::c_int as std::os::raw::c_uint) == 0
        || n.wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint) == 0
    {
        return 0 as std::os::raw::c_int
    } else {
        let mut i: std::os::raw::c_uint = 0;
        i = 5 as std::os::raw::c_int as std::os::raw::c_uint;
        while i.wrapping_mul(i) <= n {
            if n.wrapping_rem(i) == 0
                || n.wrapping_rem(i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)) == 0
            {
                return 0 as std::os::raw::c_int;
            }
            i = i.wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_uint);
        }
        return 1 as std::os::raw::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn carmichael3(mut p1: std::os::raw::c_int) {
    if is_prime(p1 as std::os::raw::c_uint) == 0 {
        return;
    }
    let mut h3: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = 0;
    let mut p2: std::os::raw::c_int = 0;
    let mut p3: std::os::raw::c_int = 0;
    h3 = 1 as std::os::raw::c_int;
    while h3 < p1 {
        d = 1 as std::os::raw::c_int;
        while d < h3 + p1 {
            if (h3 + p1) * (p1 - 1 as std::os::raw::c_int) % d == 0 as std::os::raw::c_int
                && (-p1 * p1 % h3 + h3) % h3 == d % h3
            {
                p2 = 1 as std::os::raw::c_int + (p1 - 1 as std::os::raw::c_int) * (h3 + p1) / d;
                if !(is_prime(p2 as std::os::raw::c_uint) == 0) {
                    p3 = 1 as std::os::raw::c_int + p1 * p2 / h3;
                    if !(is_prime(p3 as std::os::raw::c_uint) == 0
                        || p2 * p3 % (p1 - 1 as std::os::raw::c_int) != 1 as std::os::raw::c_int)
                    {
                        printf(
                            b"%d %d %d\n\0" as *const u8 as *const std::os::raw::c_char,
                            p1,
                            p2,
                            p3,
                        );
                    }
                }
            }
            d += 1;
            d;
        }
        h3 += 1;
        h3;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut p1: std::os::raw::c_int = 0;
    p1 = 2 as std::os::raw::c_int;
    while p1 < 62 as std::os::raw::c_int {
        carmichael3(p1);
        p1 += 1;
        p1;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

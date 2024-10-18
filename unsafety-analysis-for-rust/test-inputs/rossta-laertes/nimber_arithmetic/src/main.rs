#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type uint32_t = std::os::raw::c_uint;
#[no_mangle]
pub extern "C" fn hpo2(mut n: uint32_t) -> uint32_t {
    return n & n.wrapping_neg();
}
#[no_mangle]
pub extern "C" fn lhpo2(mut n: uint32_t) -> uint32_t {
    let mut q: uint32_t = 0 as std::os::raw::c_int as uint32_t;
    let mut m: uint32_t = hpo2(n);
    while m.wrapping_rem(2 as std::os::raw::c_int as std::os::raw::c_uint)
        == 0 as std::os::raw::c_int as std::os::raw::c_uint
    {
        m >>= 1 as std::os::raw::c_int;
        q = q.wrapping_add(1);
        q;
    }
    return q;
}
#[no_mangle]
pub extern "C" fn nimsum(mut x: uint32_t, mut y: uint32_t) -> uint32_t {
    return x ^ y;
}
#[no_mangle]
pub extern "C" fn nimprod(mut x: uint32_t, mut y: uint32_t) -> uint32_t {
    if x < 2 as std::os::raw::c_int as std::os::raw::c_uint || y < 2 as std::os::raw::c_int as std::os::raw::c_uint {
        return x.wrapping_mul(y);
    }
    let mut h: uint32_t = hpo2(x);
    if x > h {
        return nimprod(h, y) ^ nimprod(x ^ h, y);
    }
    if hpo2(y) < y {
        return nimprod(y, x);
    }
    let mut xp: uint32_t = lhpo2(x);
    let mut yp: uint32_t = lhpo2(y);
    let mut comp: uint32_t = xp & yp;
    if comp == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return x.wrapping_mul(y);
    }
    h = hpo2(comp);
    return nimprod(
        nimprod(x >> h, y >> h),
        ((3 as std::os::raw::c_int) << h.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint))
            as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_table(
    mut n: uint32_t,
    mut op: std::os::raw::c_char,
    mut func: Option::<unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t>,
) {
    printf(b" %c |\0" as *const u8 as *const std::os::raw::c_char, op as std::os::raw::c_int);
    let mut a: uint32_t = 0 as std::os::raw::c_int as uint32_t;
    while a <= n {
        printf(b"%3d\0" as *const u8 as *const std::os::raw::c_char, a);
        a = a.wrapping_add(1);
        a;
    }
    printf(b"\n--- -\0" as *const u8 as *const std::os::raw::c_char);
    let mut a_0: uint32_t = 0 as std::os::raw::c_int as uint32_t;
    while a_0 <= n {
        printf(b"---\0" as *const u8 as *const std::os::raw::c_char);
        a_0 = a_0.wrapping_add(1);
        a_0;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    let mut b: uint32_t = 0 as std::os::raw::c_int as uint32_t;
    while b <= n {
        printf(b"%2d |\0" as *const u8 as *const std::os::raw::c_char, b);
        let mut a_1: uint32_t = 0 as std::os::raw::c_int as uint32_t;
        while a_1 <= n {
            printf(
                b"%3d\0" as *const u8 as *const std::os::raw::c_char,
                func.expect("non-null function pointer")(a_1, b),
            );
            a_1 = a_1.wrapping_add(1);
            a_1;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        b = b.wrapping_add(1);
        b;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    print_table(
        15 as std::os::raw::c_int as uint32_t,
        '+' as i32 as std::os::raw::c_char,
        Some(nimsum as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t),
    );
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    print_table(
        15 as std::os::raw::c_int as uint32_t,
        '*' as i32 as std::os::raw::c_char,
        Some(nimprod as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t),
    );
    let a: uint32_t = 21508 as std::os::raw::c_int as uint32_t;
    let b: uint32_t = 42689 as std::os::raw::c_int as uint32_t;
    printf(
        b"\n%d + %d = %d\n\0" as *const u8 as *const std::os::raw::c_char,
        a,
        b,
        nimsum(a, b),
    );
    printf(b"%d * %d = %d\n\0" as *const u8 as *const std::os::raw::c_char, a, b, nimprod(a, b));
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

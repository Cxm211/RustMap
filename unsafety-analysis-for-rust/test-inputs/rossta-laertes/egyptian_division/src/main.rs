#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
}
pub type __uint64_t = std::os::raw::c_ulong;
pub type uint64_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn egyptian_division<'a1>(
    mut dividend: std::os::raw::c_ulong,
    mut divisor: std::os::raw::c_ulong,
    mut remainder: Option<&'a1 mut std::os::raw::c_ulong>,
) -> std::os::raw::c_ulong {
    static mut powers: [uint64_t; 64] = [0; 64];
    static mut doublings: [uint64_t; 64] = [0; 64];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 64 as std::os::raw::c_int {
        powers[i as usize] = ((1 as std::os::raw::c_int) << i) as uint64_t;
        doublings[i as usize] = divisor << i;
        if doublings[i as usize] > dividend {
            break;
        }
        i += 1;
        i;
    }
    let mut answer: uint64_t = 0 as std::os::raw::c_int as uint64_t;
    let mut accumulator: uint64_t = 0 as std::os::raw::c_int as uint64_t;
    i = i - 1 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        if accumulator.wrapping_add(doublings[i as usize]) <= dividend {
            accumulator = (accumulator as std::os::raw::c_ulong)
                .wrapping_add(doublings[i as usize]) as uint64_t as uint64_t;
            answer = (answer as std::os::raw::c_ulong).wrapping_add(powers[i as usize])
                as uint64_t as uint64_t;
        }
        i -= 1;
        i;
    }
    if !borrow(& remainder).is_none() {
        *borrow_mut(&mut remainder).unwrap() = dividend.wrapping_sub(accumulator);
    }
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn go(mut a: uint64_t, mut b: uint64_t) {
    let mut x: uint64_t = 0;
    let mut y: uint64_t = 0;
    x = egyptian_division(a, b, Some(&mut y));
    printf(
        b"%llu / %llu = %llu remainder %llu\n\0" as *const u8 as *const std::os::raw::c_char,
        a,
        b,
        x,
        y,
    );
    if a == b.wrapping_mul(x).wrapping_add(y) {} else {
        __assert_fail(
            b"a == b * x + y\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            43 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 28],
                &[std::os::raw::c_char; 28],
            >(b"void go(uint64_t, uint64_t)\0"))
                .as_ptr(),
        );
    }
    'c_1638: {
        if a == b.wrapping_mul(x).wrapping_add(y) {} else {
            __assert_fail(
                b"a == b * x + y\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                43 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 28],
                    &[std::os::raw::c_char; 28],
                >(b"void go(uint64_t, uint64_t)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    go(580 as std::os::raw::c_int as uint64_t, 32 as std::os::raw::c_int as uint64_t);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

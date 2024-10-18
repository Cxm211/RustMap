#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
pub type set_t = std::os::raw::c_uint;
#[no_mangle]
pub unsafe extern "C" fn show_set(mut x: set_t, mut name: *const std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    printf(b"%s is:\0" as *const u8 as *const std::os::raw::c_char, name);
    i = 0 as std::os::raw::c_int;
    while (1 as std::os::raw::c_uint) << i <= x {
        if x & (1 as std::os::raw::c_uint) << i != 0 {
            printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, i);
        }
        i += 1;
        i;
    }
    putchar('\n' as i32);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut a: set_t = 0;
    let mut b: set_t = 0;
    let mut c: set_t = 0;
    a = 0 as std::os::raw::c_int as set_t;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        a |= (1 as std::os::raw::c_uint) << i;
        i += 3 as std::os::raw::c_int;
    }
    show_set(a, b"a\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        printf(
            b"\t%d%s in set a\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            if a & (1 as std::os::raw::c_uint) << i != 0 {
                b"\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b" not\0" as *const u8 as *const std::os::raw::c_char
            },
        );
        i += 1;
        i;
    }
    b = a;
    b |= (1 as std::os::raw::c_uint) << 5 as std::os::raw::c_int;
    b |= (1 as std::os::raw::c_uint) << 10 as std::os::raw::c_int;
    b &= !((1 as std::os::raw::c_uint) << 0 as std::os::raw::c_int);
    show_set(b, b"b\0" as *const u8 as *const std::os::raw::c_char);
    show_set(a | b, b"union(a, b)\0" as *const u8 as *const std::os::raw::c_char);
    c = a & b;
    show_set(c, b"c = common(a, b)\0" as *const u8 as *const std::os::raw::c_char);
    show_set(a & !b, b"a - b\0" as *const u8 as *const std::os::raw::c_char);
    show_set(b & !a, b"b - a\0" as *const u8 as *const std::os::raw::c_char);
    printf(
        b"b is%s a subset of a\n\0" as *const u8 as *const std::os::raw::c_char,
        if b & !a == 0 {
            b"\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b" not\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    printf(
        b"c is%s a subset of a\n\0" as *const u8 as *const std::os::raw::c_char,
        if c & !a == 0 {
            b"\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b" not\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    printf(
        b"union(a, b) - common(a, b) %s union(a - b, b - a)\n\0" as *const u8
            as *const std::os::raw::c_char,
        if (a | b) & !(a & b) == a & !b | b & !a {
            b"equals\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"does not equal\0" as *const u8 as *const std::os::raw::c_char
        },
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

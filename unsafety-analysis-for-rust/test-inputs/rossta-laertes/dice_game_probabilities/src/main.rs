#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __uint64_t = std::os::raw::c_ulong;
pub type uint32_t = std::os::raw::c_uint;
pub type uint64_t = std::os::raw::c_ulong;
pub type uint = std::os::raw::c_uint;
pub type ulong = std::os::raw::c_ulong;
#[no_mangle]
pub extern "C" fn ipow(x: uint, y: uint) -> ulong {
    let mut result: ulong = 1 as std::os::raw::c_int as ulong;
    let mut i: uint = 1 as std::os::raw::c_int as uint;
    while i <= y {
        result = (result as std::os::raw::c_ulong).wrapping_mul(x as std::os::raw::c_ulong) as ulong
            as ulong;
        i = i.wrapping_add(1);
        i;
    }
    return result;
}
#[no_mangle]
pub extern "C" fn min(x: uint, y: uint) -> uint {
    return if x < y { x } else { y };
}
#[no_mangle]
pub unsafe extern "C" fn throw_die(
    n_sides: uint,
    n_dice: uint,
    s: uint,
    mut counts: *mut uint,
) {
    if n_dice == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        let ref mut fresh0 = *counts.offset(s as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
        return;
    }
    let mut i: uint = 1 as std::os::raw::c_int as uint;
    while i < n_sides.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) {
        throw_die(
            n_sides,
            n_dice.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint),
            s.wrapping_add(i),
            counts,
        );
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn beating_probability(
    n_sides1: uint,
    n_dice1: uint,
    n_sides2: uint,
    n_dice2: uint,
) -> std::os::raw::c_double {
    let len1: uint = n_sides1
        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
        .wrapping_mul(n_dice1);
    let vla = len1 as usize;
    let mut C1: Vec::<uint> = ::std::vec::from_elem(0, vla);
    let mut i: uint = 0 as std::os::raw::c_int as uint;
    while i < len1 {
        *C1.as_mut_ptr().offset(i as isize) = 0 as std::os::raw::c_int as uint;
        i = i.wrapping_add(1);
        i;
    }
    throw_die(n_sides1, n_dice1, 0 as std::os::raw::c_int as uint, C1.as_mut_ptr());
    let len2: uint = n_sides2
        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
        .wrapping_mul(n_dice2);
    let vla_0 = len2 as usize;
    let mut C2: Vec::<uint> = ::std::vec::from_elem(0, vla_0);
    let mut j: uint = 0 as std::os::raw::c_int as uint;
    while j < len2 {
        *C2.as_mut_ptr().offset(j as isize) = 0 as std::os::raw::c_int as uint;
        j = j.wrapping_add(1);
        j;
    }
    throw_die(n_sides2, n_dice2, 0 as std::os::raw::c_int as uint, C2.as_mut_ptr());
    let p12: std::os::raw::c_double = (ipow(n_sides1, n_dice1))
        .wrapping_mul(ipow(n_sides2, n_dice2)) as std::os::raw::c_double;
    let mut tot: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i_0: uint = 0 as std::os::raw::c_int as uint;
    while i_0 < len1 {
        let mut j_0: uint = 0 as std::os::raw::c_int as uint;
        while j_0 < min(i_0, len2) {
            tot
                += *C1.as_mut_ptr().offset(i_0 as isize) as std::os::raw::c_double
                    * *C2.as_mut_ptr().offset(j_0 as isize) as std::os::raw::c_double / p12;
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    return tot;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"%1.16f\n\0" as *const u8 as *const std::os::raw::c_char,
        beating_probability(
            4 as std::os::raw::c_int as uint,
            9 as std::os::raw::c_int as uint,
            6 as std::os::raw::c_int as uint,
            6 as std::os::raw::c_int as uint,
        ),
    );
    printf(
        b"%1.16f\n\0" as *const u8 as *const std::os::raw::c_char,
        beating_probability(
            10 as std::os::raw::c_int as uint,
            5 as std::os::raw::c_int as uint,
            7 as std::os::raw::c_int as uint,
            6 as std::os::raw::c_int as uint,
        ),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

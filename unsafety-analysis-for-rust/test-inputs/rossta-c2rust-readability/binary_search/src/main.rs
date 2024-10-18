#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bsearch(
    mut a: *mut std::os::raw::c_int,
    mut n: std::os::raw::c_int,
    mut x: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut j: std::os::raw::c_int = n - 1 as std::os::raw::c_int;
    while i <= j {
        let mut k: std::os::raw::c_int = i + (j - i) / 2 as std::os::raw::c_int;
        if *a.offset(k as isize) == x {
            return k
        } else if *a.offset(k as isize) < x {
            i = k + 1 as std::os::raw::c_int;
        } else {
            j = k - 1 as std::os::raw::c_int;
        }
    }
    return -(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bsearch_r(
    mut a: *mut std::os::raw::c_int,
    mut x: std::os::raw::c_int,
    mut i: std::os::raw::c_int,
    mut j: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if j < i {
        return -(1 as std::os::raw::c_int);
    }
    let mut k: std::os::raw::c_int = i + (j - i) / 2 as std::os::raw::c_int;
    if *a.offset(k as isize) == x {
        return k
    } else if *a.offset(k as isize) < x {
        return bsearch_r(a, x, k + 1 as std::os::raw::c_int, j)
    } else {
        return bsearch_r(a, x, i, k - 1 as std::os::raw::c_int)
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut a: [std::os::raw::c_int; 10] = [
        -(31 as std::os::raw::c_int),
        0 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        65 as std::os::raw::c_int,
        83 as std::os::raw::c_int,
        99 as std::os::raw::c_int,
        782 as std::os::raw::c_int,
    ];
    let mut n: std::os::raw::c_int = (::core::mem::size_of::<[std::os::raw::c_int; 10]>()
        as std::os::raw::c_ulong)
        .wrapping_div(::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    let mut x: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = bsearch(a.as_mut_ptr(), n, x);
    printf(b"%d is at index %d\n\0" as *const u8 as *const std::os::raw::c_char, x, i);
    x = 5 as std::os::raw::c_int;
    i = bsearch_r(a.as_mut_ptr(), x, 0 as std::os::raw::c_int, n - 1 as std::os::raw::c_int);
    printf(b"%d is at index %d\n\0" as *const u8 as *const std::os::raw::c_char, x, i);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

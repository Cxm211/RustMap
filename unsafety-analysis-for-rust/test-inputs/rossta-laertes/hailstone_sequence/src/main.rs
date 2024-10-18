#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
#[no_mangle]
pub unsafe extern "C" fn hailstone(
    mut n: std::os::raw::c_int,
    mut arry: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut hs: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while n != 1 as std::os::raw::c_int {
        hs += 1;
        hs;
        if !arry.is_null() {
            let fresh0 = arry;
            arry = arry.offset(1);
            *fresh0 = n;
        }
        n = if n & 1 as std::os::raw::c_int != 0 {
            3 as std::os::raw::c_int * n + 1 as std::os::raw::c_int
        } else {
            n / 2 as std::os::raw::c_int
        };
    }
    if !arry.is_null() {
        let fresh1 = arry;
        arry = arry.offset(1);
        *fresh1 = n;
    }
    return hs;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut j: std::os::raw::c_int = 0;
    let mut hmax: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut jatmax: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut arry: *mut std::os::raw::c_int = core::ptr::null_mut();
    j = 1 as std::os::raw::c_int;
    while j < 100000 as std::os::raw::c_int {
        n = hailstone(j, core::ptr::null_mut());
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
        j += 1;
        j;
    }
    n = hailstone(27 as std::os::raw::c_int, core::ptr::null_mut());
    arry = malloc(
        (n as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    n = hailstone(27 as std::os::raw::c_int, arry);
    printf(
        b"[ %d, %d, %d, %d, ...., %d, %d, %d, %d] len=%d\n\0" as *const u8
            as *const std::os::raw::c_char,
        *arry.offset(0 as std::os::raw::c_int as isize),
        *arry.offset(1 as std::os::raw::c_int as isize),
        *arry.offset(2 as std::os::raw::c_int as isize),
        *arry.offset(3 as std::os::raw::c_int as isize),
        *arry.offset((n - 4 as std::os::raw::c_int) as isize),
        *arry.offset((n - 3 as std::os::raw::c_int) as isize),
        *arry.offset((n - 2 as std::os::raw::c_int) as isize),
        *arry.offset((n - 1 as std::os::raw::c_int) as isize),
        n,
    );
    printf(b"Max %d at j= %d\n\0" as *const u8 as *const std::os::raw::c_char, hmax, jatmax);
    free(arry as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

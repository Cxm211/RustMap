#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn elem(
    mut a: *mut std::os::raw::c_int,
    mut n: size_t,
    mut e: std::os::raw::c_int,
) -> bool {
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < n {
        if *a.offset(i as isize) == e {
            return 1 as std::os::raw::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn nub(mut a: *mut std::os::raw::c_int, mut n: size_t) -> size_t {
    let mut m: size_t = 0 as std::os::raw::c_int as size_t;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < n {
        if !elem(a, m, *a.offset(i as isize)) {
            let fresh0 = m;
            m = m.wrapping_add(1);
            *a.offset(fresh0 as isize) = *a.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn nub_new<'a1>(
    mut b: Option<&'a1 mut * mut std::os::raw::c_int>,
    mut a: * mut std::os::raw::c_int,
    mut n: std::os::raw::c_ulong,
) -> std::os::raw::c_ulong {
    let mut c: *mut std::os::raw::c_int = malloc(
        n.wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    memcpy(
        c as *mut u8,
        a as *const u8,
        n.wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    );
    let mut m: std::os::raw::c_int = nub(c, n) as std::os::raw::c_int;
    *borrow_mut(&mut b).unwrap() = malloc(
        (m as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    memcpy(
        *borrow_mut(&mut b).unwrap() as *mut u8,
        c as *const u8,
        (m as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    );
    free(c as *mut u8);
    return m as size_t;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut a: [std::os::raw::c_int; 10] = [
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        15 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
    ];
    let mut b: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    let mut n: size_t = nub_new(
        Some(&mut b),
        a.as_mut_ptr(),
        (std::mem::size_of::<[std::os::raw::c_int; 10]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    );
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < n {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, *b.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    puts(b"\0" as *const u8 as *const std::os::raw::c_char);
    free(b as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

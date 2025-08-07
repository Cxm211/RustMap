#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn array_concat(
    mut a_0: *const std::os::raw::c_void,
    mut an: size_t,
    mut b_0: *const std::os::raw::c_void,
    mut bn: size_t,
    mut s: size_t,
) -> *mut std::os::raw::c_void {
    let mut p: *mut std::os::raw::c_char = malloc(s.wrapping_mul(an.wrapping_add(bn)))
        as *mut std::os::raw::c_char;
    memcpy(p as *mut std::os::raw::c_void, a_0, an.wrapping_mul(s));
    memcpy(
        p.offset(an.wrapping_mul(s) as isize) as *mut std::os::raw::c_void,
        b_0,
        bn.wrapping_mul(s),
    );
    return p as *mut std::os::raw::c_void;
}
#[no_mangle]
pub static mut a: [std::os::raw::c_int; 5] = [
    1 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    3 as std::os::raw::c_int,
    4 as std::os::raw::c_int,
    5 as std::os::raw::c_int,
];
#[no_mangle]
pub static mut b: [std::os::raw::c_int; 5] = [
    6 as std::os::raw::c_int,
    7 as std::os::raw::c_int,
    8 as std::os::raw::c_int,
    9 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_uint = 0;
    let mut c: *mut std::os::raw::c_int = array_concat(
        a.as_ptr() as *const std::os::raw::c_void,
        5 as std::os::raw::c_int as size_t,
        b.as_ptr() as *const std::os::raw::c_void,
        5 as std::os::raw::c_int as size_t,
        ::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < 10 as std::os::raw::c_int as std::os::raw::c_uint {
        printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, *c.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    free(c as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

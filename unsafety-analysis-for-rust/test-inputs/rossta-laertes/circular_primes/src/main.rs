#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type uint32_t = __uint32_t;
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut u8,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut u8,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn is_prime(mut n: uint32_t) -> bool {
    if n == 2 as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int != 0;
    }
    if n < 2 as std::os::raw::c_int as std::os::raw::c_uint
        || n.wrapping_rem(2 as std::os::raw::c_int as std::os::raw::c_uint)
            == 0 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return 0 as std::os::raw::c_int != 0;
    }
    let mut p: uint32_t = 3 as std::os::raw::c_int as uint32_t;
    while p.wrapping_mul(p) <= n {
        if n.wrapping_rem(p) == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as std::os::raw::c_int != 0;
        }
        p = (p as std::os::raw::c_uint).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
            as uint32_t as uint32_t;
    }
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cycle(mut n: uint32_t) -> uint32_t {
    let mut m: uint32_t = n;
    let mut p: uint32_t = 1 as std::os::raw::c_int as uint32_t;
    while m >= 10 as std::os::raw::c_int as std::os::raw::c_uint {
        p = (p as std::os::raw::c_uint).wrapping_mul(10 as std::os::raw::c_int as std::os::raw::c_uint)
            as uint32_t as uint32_t;
        m = (m as std::os::raw::c_uint).wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_uint)
            as uint32_t as uint32_t;
    }
    return m
        .wrapping_add(
            (10 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_mul(n.wrapping_rem(p)),
        );
}
#[no_mangle]
pub unsafe extern "C" fn is_circular_prime(mut p: uint32_t) -> bool {
    if !is_prime(p) {
        return 0 as std::os::raw::c_int != 0;
    }
    let mut p2: uint32_t = cycle(p);
    while p2 != p {
        if p2 < p || !is_prime(p2) {
            return 0 as std::os::raw::c_int != 0;
        }
        p2 = cycle(p2);
    }
    return 1 as std::os::raw::c_int != 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

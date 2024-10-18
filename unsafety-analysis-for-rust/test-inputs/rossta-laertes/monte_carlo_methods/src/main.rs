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
    fn rand() -> std::os::raw::c_int;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
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
impl std::default::Default for _IO_FILE {
    fn default() -> Self {
        _IO_FILE {
        _flags: std::os::raw::c_int::default(),
        _IO_read_ptr: 0 as * mut std::os::raw::c_char,
        _IO_read_end: 0 as * mut std::os::raw::c_char,
        _IO_read_base: 0 as * mut std::os::raw::c_char,
        _IO_write_base: 0 as * mut std::os::raw::c_char,
        _IO_write_ptr: 0 as * mut std::os::raw::c_char,
        _IO_write_end: 0 as * mut std::os::raw::c_char,
        _IO_buf_base: 0 as * mut std::os::raw::c_char,
        _IO_buf_end: 0 as * mut std::os::raw::c_char,
        _IO_save_base: 0 as * mut std::os::raw::c_char,
        _IO_backup_base: 0 as * mut std::os::raw::c_char,
        _IO_save_end: 0 as * mut std::os::raw::c_char,
        _markers: 0 as * mut crate::_IO_marker,
        _chain: core::ptr::null_mut(),
        _fileno: std::os::raw::c_int::default(),
        _flags2: std::os::raw::c_int::default(),
        _old_offset: std::os::raw::c_long::default(),
        _cur_column: std::os::raw::c_ushort::default(),
        _vtable_offset: std::os::raw::c_schar::default(),
        _shortbuf: [std::os::raw::c_char::default(); 1],
        _lock: 0 as * mut u8,
        _offset: std::os::raw::c_long::default(),
        _codecvt: 0 as * mut crate::_IO_codecvt,
        _wide_data: 0 as * mut crate::_IO_wide_data,
        _freeres_list: core::ptr::null_mut(),
        _freeres_buf: 0 as * mut u8,
        __pad5: std::os::raw::c_ulong::default(),
        _mode: std::os::raw::c_int::default(),
        _unused2: [std::os::raw::c_char::default(); 20]
        }
    }
}

pub type _IO_lock_t = ();
pub type FILE = crate::_IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn pi(mut tolerance: std::os::raw::c_double) -> std::os::raw::c_double {
    let mut x: std::os::raw::c_double = 0.;
    let mut y: std::os::raw::c_double = 0.;
    let mut val: std::os::raw::c_double = 0.;
    let mut error: std::os::raw::c_double = 0.;
    let mut sampled: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut hit: std::os::raw::c_ulong = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut i: std::os::raw::c_ulong = 0;
    loop {
        i = 1000000 as std::os::raw::c_int as std::os::raw::c_ulong;
        while i != 0 {
            x = rand() as std::os::raw::c_double
                / (2147483647 as std::os::raw::c_int as std::os::raw::c_double + 1.0f64);
            y = rand() as std::os::raw::c_double
                / (2147483647 as std::os::raw::c_int as std::os::raw::c_double + 1.0f64);
            if x * x + y * y < 1 as std::os::raw::c_int as std::os::raw::c_double {
                hit = hit.wrapping_add(1);
                hit;
            }
            i = i.wrapping_sub(1);
            i;
            sampled = sampled.wrapping_add(1);
            sampled;
        }
        val = hit as std::os::raw::c_double / sampled as std::os::raw::c_double;
        error = sqrt(
            val * (1 as std::os::raw::c_int as std::os::raw::c_double - val) / sampled as std::os::raw::c_double,
        ) * 4 as std::os::raw::c_int as std::os::raw::c_double;
        val *= 4 as std::os::raw::c_int as std::os::raw::c_double;
        fprintf(
            stderr,
            b"Pi = %f +/- %5.3e at %ldM samples.\r\0" as *const u8
                as *const std::os::raw::c_char,
            val,
            error,
            sampled.wrapping_div(1000000 as std::os::raw::c_int as std::os::raw::c_ulong),
        );
        if !(hit == 0 || error > tolerance) {
            break;
        }
    }
    return val;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(b"Pi is %f\n\0" as *const u8 as *const std::os::raw::c_char, pi(3e-4f64));
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

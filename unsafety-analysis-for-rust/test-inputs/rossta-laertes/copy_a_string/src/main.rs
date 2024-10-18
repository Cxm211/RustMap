#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
    static mut stderr: *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fputs(__s: *const std::os::raw::c_char, __stream: *mut FILE) -> std::os::raw::c_int;
    fn perror(__s: *const std::os::raw::c_char);
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strdup(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
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
unsafe fn main_0() -> std::os::raw::c_int {
    let mut len: size_t = 0;
    let mut src: [std::os::raw::c_char; 6] = *std::mem::transmute::<
        &[u8; 6],
        &mut [std::os::raw::c_char; 6],
    >(b"Hello\0");
    let mut dst1: [std::os::raw::c_char; 80] = [0; 80];
    let mut dst2: [std::os::raw::c_char; 80] = [0; 80];
    let mut dst3: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut ref_0: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    strcpy(dst1.as_mut_ptr(), src.as_mut_ptr());
    len = strlen(src.as_mut_ptr());
    if len >= std::mem::size_of::<[std::os::raw::c_char; 80]>() as std::os::raw::c_ulong {
        fputs(
            b"The buffer is too small!\n\0" as *const u8 as *const std::os::raw::c_char,
            stderr,
        );
        exit(1 as std::os::raw::c_int);
    }
    memcpy(
        dst2.as_mut_ptr() as *mut u8,
        src.as_mut_ptr() as *const u8,
        len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    dst3 = strdup(src.as_mut_ptr());
    if dst3.is_null() {
        perror(b"strdup\0" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    ref_0 = src.as_mut_ptr();
    memset(
        src.as_mut_ptr() as *mut u8,
        '-' as i32,
        5 as std::os::raw::c_int as std::os::raw::c_ulong,
    );
    printf(b" src: %s\n\0" as *const u8 as *const std::os::raw::c_char, src.as_mut_ptr());
    printf(b"dst1: %s\n\0" as *const u8 as *const std::os::raw::c_char, dst1.as_mut_ptr());
    printf(b"dst2: %s\n\0" as *const u8 as *const std::os::raw::c_char, dst2.as_mut_ptr());
    printf(b"dst3: %s\n\0" as *const u8 as *const std::os::raw::c_char, dst3);
    printf(b" ref: %s\n\0" as *const u8 as *const std::os::raw::c_char, ref_0);
    free(dst3 as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

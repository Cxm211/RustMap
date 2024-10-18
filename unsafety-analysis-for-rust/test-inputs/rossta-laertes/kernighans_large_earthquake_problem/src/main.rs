#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn getline(
        __lineptr: *mut *mut std::os::raw::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strrchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    fn atof(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_double;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
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
pub type ssize_t = std::os::raw::c_long;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut len: size_t = 0 as std::os::raw::c_int as size_t;
    let mut read: ssize_t = 0;
    let mut lw: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut lt: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    fp = fopen(
        b"data.txt\0" as *const u8 as *const std::os::raw::c_char,
        b"r\0" as *const u8 as *const std::os::raw::c_char,
    );
    if fp.is_null() {
        printf(b"Unable to open file\n\0" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    printf(
        b"Those earthquakes with a magnitude > 6.0 are:\n\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    loop {
        read = getline(&mut line, &mut len, fp);
        if !(read != -(1 as std::os::raw::c_int) as std::os::raw::c_long) {
            break;
        }
        if read < 2 as std::os::raw::c_int as std::os::raw::c_long {
            continue;
        }
        lw = strrchr(line, ' ' as i32);
        lt = strrchr(line, '\t' as i32);
        if lw.is_null() && lt.is_null() {
            continue;
        }
        if lt > lw {
            lw = lt;
        }
        if atof(lw.offset(1 as std::os::raw::c_int as isize)) > 6.0f64 {
            printf(b"%s\0" as *const u8 as *const std::os::raw::c_char, line);
        }
    }
    fclose(fp);
    if !line.is_null() {
        free(line as *mut u8);
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

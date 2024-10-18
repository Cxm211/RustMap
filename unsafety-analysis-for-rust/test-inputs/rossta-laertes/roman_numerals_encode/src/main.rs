#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: std::os::raw::c_int) -> !;
    static mut stderr: *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fputs(__s: *const std::os::raw::c_char, __stream: *mut FILE) -> std::os::raw::c_int;
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
pub unsafe extern "C" fn roman(mut s: *mut std::os::raw::c_char, mut n: std::os::raw::c_uint) {
    if n == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        fputs(
            b"Roman numeral for zero requested.\0" as *const u8 as *const std::os::raw::c_char,
            stderr,
        );
        exit(1 as std::os::raw::c_int);
    }
    while n >= 1000 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh0 = s;
        s = s.offset(1);
        *fresh0 = 'M' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(1000 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 900 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh1 = s;
        s = s.offset(1);
        *fresh1 = 'C' as i32 as std::os::raw::c_char;
        let fresh2 = s;
        s = s.offset(1);
        *fresh2 = 'M' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(900 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 500 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh3 = s;
        s = s.offset(1);
        *fresh3 = 'D' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(500 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 400 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh4 = s;
        s = s.offset(1);
        *fresh4 = 'C' as i32 as std::os::raw::c_char;
        let fresh5 = s;
        s = s.offset(1);
        *fresh5 = 'D' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(400 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    while n >= 100 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh6 = s;
        s = s.offset(1);
        *fresh6 = 'C' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(100 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 90 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh7 = s;
        s = s.offset(1);
        *fresh7 = 'X' as i32 as std::os::raw::c_char;
        let fresh8 = s;
        s = s.offset(1);
        *fresh8 = 'C' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(90 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 50 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh9 = s;
        s = s.offset(1);
        *fresh9 = 'L' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(50 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 40 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh10 = s;
        s = s.offset(1);
        *fresh10 = 'X' as i32 as std::os::raw::c_char;
        let fresh11 = s;
        s = s.offset(1);
        *fresh11 = 'L' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(40 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    while n >= 10 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh12 = s;
        s = s.offset(1);
        *fresh12 = 'X' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(10 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 9 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh13 = s;
        s = s.offset(1);
        *fresh13 = 'I' as i32 as std::os::raw::c_char;
        let fresh14 = s;
        s = s.offset(1);
        *fresh14 = 'X' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(9 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 5 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh15 = s;
        s = s.offset(1);
        *fresh15 = 'V' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(5 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    if n >= 4 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh16 = s;
        s = s.offset(1);
        *fresh16 = 'I' as i32 as std::os::raw::c_char;
        let fresh17 = s;
        s = s.offset(1);
        *fresh17 = 'V' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    while n >= 1 as std::os::raw::c_int as std::os::raw::c_uint {
        let fresh18 = s;
        s = s.offset(1);
        *fresh18 = 'I' as i32 as std::os::raw::c_char;
        n = n.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    *s = 0 as std::os::raw::c_int as std::os::raw::c_char;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut buffer: [std::os::raw::c_char; 16] = [0; 16];
    let mut i: std::os::raw::c_uint = 0;
    i = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < 4000 as std::os::raw::c_int as std::os::raw::c_uint {
        roman(buffer.as_mut_ptr(), i);
        printf(
            b"%4u: %s\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            buffer.as_mut_ptr(),
        );
        i = i.wrapping_add(1);
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

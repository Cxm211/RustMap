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
    fn fgetc(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fputc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint32_t = std::os::raw::c_uint;
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
pub type uint8_t = std::os::raw::c_uchar;
pub type uint32_t = std::os::raw::c_uint;
pub type byte = std::os::raw::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_io_t {
    pub fp: * mut crate::_IO_FILE,
    pub accu: std::os::raw::c_uint,
    pub bits: std::os::raw::c_int,
}
impl std::default::Default for bit_io_t {
    fn default() -> Self {
        bit_io_t {
        fp: core::ptr::null_mut(),
        accu: std::os::raw::c_uint::default(),
        bits: std::os::raw::c_int::default()
        }
    }
}

pub type bit_filter = * mut crate::bit_io_t;
#[no_mangle]
pub unsafe extern "C" fn b_attach(mut f: *mut FILE) -> bit_filter {
    let mut b: bit_filter = malloc(std::mem::size_of::<bit_io_t>() as std::os::raw::c_ulong)
        as bit_filter;
    (*b).accu = 0 as std::os::raw::c_int as uint32_t;
    (*b).bits = (*b).accu as std::os::raw::c_int;
    (*b).fp = f;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn b_write(
    mut buf: *mut byte,
    mut n_bits: size_t,
    mut shift: size_t,
    mut bf: bit_filter,
) {
    let mut accu: uint32_t = (*bf).accu;
    let mut bits: std::os::raw::c_int = (*bf).bits;
    buf = buf.offset(shift.wrapping_div(8 as std::os::raw::c_int as std::os::raw::c_ulong) as isize);
    shift = (shift as std::os::raw::c_ulong).wrapping_rem(8 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    while n_bits != 0 || bits >= 8 as std::os::raw::c_int {
        while bits >= 8 as std::os::raw::c_int {
            bits -= 8 as std::os::raw::c_int;
            fputc((accu >> bits) as std::os::raw::c_int, (*bf).fp);
            accu &= (((1 as std::os::raw::c_int) << bits) - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
        }
        while bits < 8 as std::os::raw::c_int && n_bits != 0 {
            accu = accu << 1 as std::os::raw::c_int
                | ((128 as std::os::raw::c_int >> shift & *buf as std::os::raw::c_int)
                    >> (7 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_sub(shift))
                    as std::os::raw::c_uint;
            n_bits = n_bits.wrapping_sub(1);
            n_bits;
            bits += 1;
            bits;
            shift = shift.wrapping_add(1);
            if shift == 8 as std::os::raw::c_int as std::os::raw::c_ulong {
                shift = 0 as std::os::raw::c_int as size_t;
                buf = buf.offset(1);
                buf;
            }
        }
    }
    (*bf).accu = accu;
    (*bf).bits = bits;
}
#[no_mangle]
pub unsafe extern "C" fn b_read(
    mut buf: *mut byte,
    mut n_bits: size_t,
    mut shift: size_t,
    mut bf: bit_filter,
) -> size_t {
    let mut accu: uint32_t = (*bf).accu;
    let mut bits: std::os::raw::c_int = (*bf).bits;
    let mut mask: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    buf = buf.offset(shift.wrapping_div(8 as std::os::raw::c_int as std::os::raw::c_ulong) as isize);
    shift = (shift as std::os::raw::c_ulong).wrapping_rem(8 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    while n_bits != 0 {
        while bits != 0 && n_bits != 0 {
            mask = 128 as std::os::raw::c_int >> shift;
            if accu & ((1 as std::os::raw::c_int) << bits - 1 as std::os::raw::c_int) as std::os::raw::c_uint
                != 0
            {
                *buf = (*buf as std::os::raw::c_int | mask) as byte;
            } else {
                *buf = (*buf as std::os::raw::c_int & !mask) as byte;
            }
            n_bits = n_bits.wrapping_sub(1);
            n_bits;
            bits -= 1;
            bits;
            shift = shift.wrapping_add(1);
            if shift >= 8 as std::os::raw::c_int as std::os::raw::c_ulong {
                shift = 0 as std::os::raw::c_int as size_t;
                buf = buf.offset(1);
                buf;
            }
        }
        if n_bits == 0 {
            break;
        }
        accu = accu << 8 as std::os::raw::c_int | fgetc((*bf).fp) as std::os::raw::c_uint;
        bits += 8 as std::os::raw::c_int;
    }
    (*bf).accu = accu;
    (*bf).bits = bits;
    return i as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn b_detach(mut bf: bit_filter) {
    if (*bf).bits != 0 {
        (*bf).accu <<= 8 as std::os::raw::c_int - (*bf).bits;
        fputc((*bf).accu as std::os::raw::c_int, (*bf).fp);
    }
    free(bf as *mut u8);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut s: [std::os::raw::c_uchar; 12] = *std::mem::transmute::<
        &[u8; 12],
        &mut [std::os::raw::c_uchar; 12],
    >(b"abcdefghijk\0");
    let mut s2: [std::os::raw::c_uchar; 11] = [
        0 as std::os::raw::c_int as std::os::raw::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: std::os::raw::c_int = 0;
    let mut f: *mut FILE = fopen(
        b"test.bin\0" as *const u8 as *const std::os::raw::c_char,
        b"wb\0" as *const u8 as *const std::os::raw::c_char,
    );
    let mut b: bit_filter = b_attach(f);
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        b_write(
            s.as_mut_ptr().offset(i as isize),
            7 as std::os::raw::c_int as size_t,
            1 as std::os::raw::c_int as size_t,
            b,
        );
        i += 1;
        i;
    }
    b_detach(b);
    fclose(f);
    f = fopen(
        b"test.bin\0" as *const u8 as *const std::os::raw::c_char,
        b"rb\0" as *const u8 as *const std::os::raw::c_char,
    );
    b = b_attach(f);
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        b_read(
            s2.as_mut_ptr().offset(i as isize),
            7 as std::os::raw::c_int as size_t,
            1 as std::os::raw::c_int as size_t,
            b,
        );
        i += 1;
        i;
    }
    b_detach(b);
    fclose(f);
    printf(b"%10s\n\0" as *const u8 as *const std::os::raw::c_char, s2.as_mut_ptr());
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

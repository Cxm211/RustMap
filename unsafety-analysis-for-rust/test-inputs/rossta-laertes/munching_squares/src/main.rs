#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fwrite(
        _: *const u8,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fmod(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
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
pub unsafe extern "C" fn hue_to_rgb(
    mut hue: std::os::raw::c_double,
    mut sat: std::os::raw::c_double,
    mut p: *mut std::os::raw::c_uchar,
) {
    let mut x: std::os::raw::c_double = 0.;
    let mut c: std::os::raw::c_int = (255 as std::os::raw::c_int as std::os::raw::c_double * sat) as std::os::raw::c_int;
    hue /= 60 as std::os::raw::c_int as std::os::raw::c_double;
    x = (1 as std::os::raw::c_int as std::os::raw::c_double
        - fabs(
            fmod(hue, 2 as std::os::raw::c_int as std::os::raw::c_double)
                - 1 as std::os::raw::c_int as std::os::raw::c_double,
        )) * 255 as std::os::raw::c_int as std::os::raw::c_double;
    match hue as std::os::raw::c_int {
        0 => {
            *p.offset(0 as std::os::raw::c_int as isize) = c as std::os::raw::c_uchar;
            *p.offset(1 as std::os::raw::c_int as isize) = x as std::os::raw::c_uchar;
            *p.offset(2 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            return;
        }
        1 => {
            *p.offset(0 as std::os::raw::c_int as isize) = x as std::os::raw::c_uchar;
            *p.offset(1 as std::os::raw::c_int as isize) = c as std::os::raw::c_uchar;
            *p.offset(2 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            return;
        }
        2 => {
            *p.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            *p.offset(1 as std::os::raw::c_int as isize) = c as std::os::raw::c_uchar;
            *p.offset(2 as std::os::raw::c_int as isize) = x as std::os::raw::c_uchar;
            return;
        }
        3 => {
            *p.offset(0 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            *p.offset(1 as std::os::raw::c_int as isize) = x as std::os::raw::c_uchar;
            *p.offset(2 as std::os::raw::c_int as isize) = c as std::os::raw::c_uchar;
            return;
        }
        4 => {
            *p.offset(0 as std::os::raw::c_int as isize) = x as std::os::raw::c_uchar;
            *p.offset(1 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            *p.offset(2 as std::os::raw::c_int as isize) = c as std::os::raw::c_uchar;
            return;
        }
        5 => {
            *p.offset(0 as std::os::raw::c_int as isize) = c as std::os::raw::c_uchar;
            *p.offset(1 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            *p.offset(2 as std::os::raw::c_int as isize) = x as std::os::raw::c_uchar;
            return;
        }
        _ => {}
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let size: std::os::raw::c_int = 512 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut colors: *mut std::os::raw::c_uchar = malloc(
        (size * 3 as std::os::raw::c_int) as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_uchar;
    let mut pix: *mut std::os::raw::c_uchar = malloc(
        (size * size * 3 as std::os::raw::c_int) as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_uchar;
    let mut p: *mut std::os::raw::c_uchar = 0 as *mut std::os::raw::c_uchar;
    let mut fp: *mut FILE = 0 as *mut FILE;
    i = 0 as std::os::raw::c_int;
    while i < size {
        hue_to_rgb(
            i as std::os::raw::c_double * 240.0f64 / size as std::os::raw::c_double,
            i as std::os::raw::c_double * 1.0f64 / size as std::os::raw::c_double,
            colors.offset((3 as std::os::raw::c_int * i) as isize),
        );
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    p = pix;
    while i < size {
        j = 0 as std::os::raw::c_int;
        while j < size {
            memcpy(
                p as *mut u8,
                colors.offset(((i ^ j) * 3 as std::os::raw::c_int) as isize)
                    as *const u8,
                3 as std::os::raw::c_int as std::os::raw::c_ulong,
            );
            j += 1;
            j;
            p = p.offset(3 as std::os::raw::c_int as isize);
        }
        i += 1;
        i;
    }
    fp = fopen(
        b"xor.ppm\0" as *const u8 as *const std::os::raw::c_char,
        b"wb\0" as *const u8 as *const std::os::raw::c_char,
    );
    fprintf(fp, b"P6\n%d %d\n255\n\0" as *const u8 as *const std::os::raw::c_char, size, size);
    fwrite(
        pix as *const u8,
        (size * size * 3 as std::os::raw::c_int) as std::os::raw::c_ulong,
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        fp,
    );
    fclose(fp);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

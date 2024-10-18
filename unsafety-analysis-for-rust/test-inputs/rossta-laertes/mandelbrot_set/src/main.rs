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
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fwrite(
        _: *const u8,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
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
    let mut iX: std::os::raw::c_int = 0;
    let mut iY: std::os::raw::c_int = 0;
    let iXmax: std::os::raw::c_int = 800 as std::os::raw::c_int;
    let iYmax: std::os::raw::c_int = 800 as std::os::raw::c_int;
    let mut Cx: std::os::raw::c_double = 0.;
    let mut Cy: std::os::raw::c_double = 0.;
    let CxMin: std::os::raw::c_double = -2.5f64;
    let CxMax: std::os::raw::c_double = 1.5f64;
    let CyMin: std::os::raw::c_double = -2.0f64;
    let CyMax: std::os::raw::c_double = 2.0f64;
    let mut PixelWidth: std::os::raw::c_double = (CxMax - CxMin) / iXmax as std::os::raw::c_double;
    let mut PixelHeight: std::os::raw::c_double = (CyMax - CyMin) / iYmax as std::os::raw::c_double;
    let MaxColorComponentValue: std::os::raw::c_int = 255 as std::os::raw::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut filename: *mut std::os::raw::c_char = b"new1.ppm\0" as *const u8
        as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut comment: *mut std::os::raw::c_char = b"# \0" as *const u8 as *const std::os::raw::c_char
        as *mut std::os::raw::c_char;
    static mut color: [std::os::raw::c_uchar; 3] = [0; 3];
    let mut Zx: std::os::raw::c_double = 0.;
    let mut Zy: std::os::raw::c_double = 0.;
    let mut Zx2: std::os::raw::c_double = 0.;
    let mut Zy2: std::os::raw::c_double = 0.;
    let mut Iteration: std::os::raw::c_int = 0;
    let IterationMax: std::os::raw::c_int = 200 as std::os::raw::c_int;
    let EscapeRadius: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double;
    let mut ER2: std::os::raw::c_double = EscapeRadius * EscapeRadius;
    fp = fopen(filename, b"wb\0" as *const u8 as *const std::os::raw::c_char);
    fprintf(
        fp,
        b"P6\n %s\n %d\n %d\n %d\n\0" as *const u8 as *const std::os::raw::c_char,
        comment,
        iXmax,
        iYmax,
        MaxColorComponentValue,
    );
    iY = 0 as std::os::raw::c_int;
    while iY < iYmax {
        Cy = CyMin + iY as std::os::raw::c_double * PixelHeight;
        if fabs(Cy) < PixelHeight / 2 as std::os::raw::c_int as std::os::raw::c_double {
            Cy = 0.0f64;
        }
        iX = 0 as std::os::raw::c_int;
        while iX < iXmax {
            Cx = CxMin + iX as std::os::raw::c_double * PixelWidth;
            Zx = 0.0f64;
            Zy = 0.0f64;
            Zx2 = Zx * Zx;
            Zy2 = Zy * Zy;
            Iteration = 0 as std::os::raw::c_int;
            while Iteration < IterationMax && Zx2 + Zy2 < ER2 {
                Zy = 2 as std::os::raw::c_int as std::os::raw::c_double * Zx * Zy + Cy;
                Zx = Zx2 - Zy2 + Cx;
                Zx2 = Zx * Zx;
                Zy2 = Zy * Zy;
                Iteration += 1;
                Iteration;
            }
            if Iteration == IterationMax {
                color[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                color[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                color[2 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            } else {
                color[0 as std::os::raw::c_int as usize] = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
                color[1 as std::os::raw::c_int as usize] = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
                color[2 as std::os::raw::c_int as usize] = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
            }
            fwrite(
                color.as_mut_ptr() as *const u8,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                3 as std::os::raw::c_int as std::os::raw::c_ulong,
                fp,
            );
            iX += 1;
            iX;
        }
        iY += 1;
        iY;
    }
    fclose(fp);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

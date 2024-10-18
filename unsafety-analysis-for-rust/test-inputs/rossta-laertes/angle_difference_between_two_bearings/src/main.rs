#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_double;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fscanf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fmod(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
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
pub unsafe extern "C" fn processFile(mut name: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut records: std::os::raw::c_int = 0;
    let mut diff: std::os::raw::c_double = 0.;
    let mut b1: std::os::raw::c_double = 0.;
    let mut b2: std::os::raw::c_double = 0.;
    let mut fp: *mut FILE = fopen(name, b"r\0" as *const u8 as *const std::os::raw::c_char);
    fscanf(
        fp,
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        &mut records as *mut std::os::raw::c_int,
    );
    i = 0 as std::os::raw::c_int;
    while i < records {
        fscanf(
            fp,
            b"%lf%lf\0" as *const u8 as *const std::os::raw::c_char,
            &mut b1 as *mut std::os::raw::c_double,
            &mut b2 as *mut std::os::raw::c_double,
        );
        diff = fmod(b2 - b1, 360.0f64);
        printf(
            b"\nDifference between b2(%lf) and b1(%lf) is %lf\0" as *const u8
                as *const std::os::raw::c_char,
            b2,
            b1,
            if diff < -(180 as std::os::raw::c_int) as std::os::raw::c_double {
                diff + 360 as std::os::raw::c_int as std::os::raw::c_double
            } else if diff >= 180 as std::os::raw::c_int as std::os::raw::c_double {
                diff - 360 as std::os::raw::c_int as std::os::raw::c_double
            } else {
                diff
            },
        );
        i += 1;
        i;
    }
    fclose(fp);
}
unsafe fn main_0(
    mut argC: std::os::raw::c_int,
    mut argV: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut diff: std::os::raw::c_double = 0.;
    if argC < 2 as std::os::raw::c_int {
        printf(
            b"Usage : %s <bearings separated by a space OR full file name which contains the bearing list>\0"
                as *const u8 as *const std::os::raw::c_char,
            *argV.offset(0 as std::os::raw::c_int as isize),
        );
    } else if argC == 2 as std::os::raw::c_int {
        processFile(*argV.offset(1 as std::os::raw::c_int as isize));
    } else {
        diff = fmod(
            atof(*argV.offset(2 as std::os::raw::c_int as isize))
                - atof(*argV.offset(1 as std::os::raw::c_int as isize)),
            360.0f64,
        );
        printf(
            b"Difference between b2(%s) and b1(%s) is %lf\0" as *const u8
                as *const std::os::raw::c_char,
            *argV.offset(2 as std::os::raw::c_int as isize),
            *argV.offset(1 as std::os::raw::c_int as isize),
            if diff < -(180 as std::os::raw::c_int) as std::os::raw::c_double {
                diff + 360 as std::os::raw::c_int as std::os::raw::c_double
            } else if diff >= 180 as std::os::raw::c_int as std::os::raw::c_double {
                diff - 360 as std::os::raw::c_int as std::os::raw::c_double
            } else {
                diff
            },
        );
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

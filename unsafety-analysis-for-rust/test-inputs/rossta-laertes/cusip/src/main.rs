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
    fn fscanf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
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
pub unsafe extern "C" fn cusipCheck(mut str: *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut v: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        if *str.offset(i as isize) as std::os::raw::c_int >= '0' as i32
            && *str.offset(i as isize) as std::os::raw::c_int <= '9' as i32
        {
            v = *str.offset(i as isize) as std::os::raw::c_int - '0' as i32;
        } else if *str.offset(i as isize) as std::os::raw::c_int >= 'A' as i32
            && *str.offset(i as isize) as std::os::raw::c_int <= 'Z' as i32
        {
            v = *str.offset(i as isize) as std::os::raw::c_int - 'A' as i32 + 10 as std::os::raw::c_int;
        } else if *str.offset(i as isize) as std::os::raw::c_int == '*' as i32 {
            v = 36 as std::os::raw::c_int;
        } else if *str.offset(i as isize) as std::os::raw::c_int == '@' as i32 {
            v = 37 as std::os::raw::c_int;
        } else if *str.offset(i as isize) as std::os::raw::c_int == '#' as i32 {
            v = 38 as std::os::raw::c_int;
        }
        if i % 2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
            v *= 2 as std::os::raw::c_int;
        }
        sum += v / 10 as std::os::raw::c_int + v % 10 as std::os::raw::c_int;
        i += 1;
        i;
    }
    return (10 as std::os::raw::c_int - sum % 10 as std::os::raw::c_int) % 10 as std::os::raw::c_int;
}
unsafe fn main_0(
    mut argC: std::os::raw::c_int,
    mut argV: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut cusipStr: [std::os::raw::c_char; 10] = [0; 10];
    let mut i: std::os::raw::c_int = 0;
    let mut numLines: std::os::raw::c_int = 0;
    if argC == 1 as std::os::raw::c_int {
        printf(
            b"Usage : %s <full path of CUSIP Data file>\0" as *const u8
                as *const std::os::raw::c_char,
            *argV.offset(0 as std::os::raw::c_int as isize),
        );
    } else {
        let mut fp: *mut FILE = fopen(
            *argV.offset(1 as std::os::raw::c_int as isize),
            b"r\0" as *const u8 as *const std::os::raw::c_char,
        );
        fscanf(
            fp,
            b"%d\0" as *const u8 as *const std::os::raw::c_char,
            &mut numLines as *mut std::os::raw::c_int,
        );
        printf(b"CUSIP       Verdict\n\0" as *const u8 as *const std::os::raw::c_char);
        printf(b"-------------------\0" as *const u8 as *const std::os::raw::c_char);
        i = 0 as std::os::raw::c_int;
        while i < numLines {
            fscanf(
                fp,
                b"%s\0" as *const u8 as *const std::os::raw::c_char,
                cusipStr.as_mut_ptr(),
            );
            printf(
                b"\n%s : %s\0" as *const u8 as *const std::os::raw::c_char,
                cusipStr.as_mut_ptr(),
                if cusipCheck(cusipStr.as_mut_ptr())
                    == cusipStr[8 as std::os::raw::c_int as usize] as std::os::raw::c_int - '0' as i32
                {
                    b"Valid\0" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"Invalid\0" as *const u8 as *const std::os::raw::c_char
                },
            );
            i += 1;
            i;
        }
        fclose(fp);
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

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn perror(__s: *const std::os::raw::c_char);
    fn opendir(__name: *const std::os::raw::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> std::os::raw::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __ino_t = std::os::raw::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: std::os::raw::c_ushort,
    pub d_type: std::os::raw::c_uchar,
    pub d_name: [std::os::raw::c_char; 256],
}
impl std::default::Default for dirent {
    fn default() -> Self {
        dirent {
        d_ino: std::os::raw::c_ulong::default(),
        d_off: std::os::raw::c_long::default(),
        d_reclen: std::os::raw::c_ushort::default(),
        d_type: std::os::raw::c_uchar::default(),
        d_name: [std::os::raw::c_char::default(); 256]
        }
    }
}

pub type DIR = crate::__dirstream;
#[no_mangle]
pub unsafe extern "C" fn dir_empty(mut path: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut ent: *mut dirent = core::ptr::null_mut();
    let mut ret: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut d: *mut DIR = opendir(path);
    if d.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const std::os::raw::c_char, path);
        perror(b"\0" as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int);
    }
    loop {
        ent = readdir(d);
        if ent.is_null() {
            break;
        }
        if strcmp(
            ((*ent).d_name).as_mut_ptr(),
            b".\0" as *const u8 as *const std::os::raw::c_char,
        ) == 0
            || strcmp(
                ((*ent).d_name).as_mut_ptr(),
                b"..\0" as *const u8 as *const std::os::raw::c_char,
            ) == 0
        {
            continue;
        }
        ret = 0 as std::os::raw::c_int;
        break;
    }
    closedir(d);
    return ret;
}
unsafe fn main_0(mut c: std::os::raw::c_int, mut v: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    if c < 2 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int);
    }
    i = 1 as std::os::raw::c_int;
    while i < c {
        ret = dir_empty(*v.offset(i as isize));
        if ret >= 0 as std::os::raw::c_int {
            printf(
                b"%s: %sempty\n\0" as *const u8 as *const std::os::raw::c_char,
                *v.offset(i as isize),
                if ret != 0 {
                    b"\0" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"not \0" as *const u8 as *const std::os::raw::c_char
                },
            );
        }
        i += 1;
        i;
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

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fputc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    fn perror(__s: *const std::os::raw::c_char);
}
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type size_t = std::os::raw::c_ulong;
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
static mut rot13_table: [std::os::raw::c_char; 256] = [0; 256];
unsafe extern "C" fn init_rot13_table() {
    static mut upper: [std::os::raw::c_uchar; 27] = unsafe {
        *std::mem::transmute::<
            &[u8; 27],
            &[std::os::raw::c_uchar; 27],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0")
    };
    static mut lower: [std::os::raw::c_uchar; 27] = unsafe {
        *std::mem::transmute::<
            &[u8; 27],
            &[std::os::raw::c_uchar; 27],
        >(b"abcdefghijklmnopqrstuvwxyz\0")
    };
    let mut ch: std::os::raw::c_int = '\0' as i32;
    while ch <= 127 as std::os::raw::c_int * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int {
        rot13_table[ch as usize] = ch as std::os::raw::c_char;
        ch += 1;
        ch;
    }
    let mut p: *const std::os::raw::c_uchar = upper.as_ptr();
    while *p.offset(13 as std::os::raw::c_int as isize) as std::os::raw::c_int != '\0' as i32 {
        rot13_table[*p.offset(0 as std::os::raw::c_int as isize)
            as usize] = *p.offset(13 as std::os::raw::c_int as isize) as std::os::raw::c_char;
        rot13_table[*p.offset(13 as std::os::raw::c_int as isize)
            as usize] = *p.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_char;
        p = p.offset(1);
        p;
    }
    let mut p_0: *const std::os::raw::c_uchar = lower.as_ptr();
    while *p_0.offset(13 as std::os::raw::c_int as isize) as std::os::raw::c_int != '\0' as i32 {
        rot13_table[*p_0.offset(0 as std::os::raw::c_int as isize)
            as usize] = *p_0.offset(13 as std::os::raw::c_int as isize) as std::os::raw::c_char;
        rot13_table[*p_0.offset(13 as std::os::raw::c_int as isize)
            as usize] = *p_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_char;
        p_0 = p_0.offset(1);
        p_0;
    }
}
unsafe extern "C" fn rot13_file(mut fp: *mut FILE) {
    let mut ch: std::os::raw::c_int = 0;
    loop {
        ch = fgetc(fp);
        if !(ch != -(1 as std::os::raw::c_int)) {
            break;
        }
        fputc(rot13_table[ch as usize] as std::os::raw::c_int, stdout);
    };
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    init_rot13_table();
    if argc > 1 as std::os::raw::c_int {
        let mut i: std::os::raw::c_int = 1 as std::os::raw::c_int;
        while i < argc {
            let mut fp: *mut FILE = fopen(
                *argv.offset(i as isize),
                b"r\0" as *const u8 as *const std::os::raw::c_char,
            );
            if fp.is_null() {
                perror(*argv.offset(i as isize));
                return 1 as std::os::raw::c_int;
            }
            rot13_file(fp);
            fclose(fp);
            i += 1;
            i;
        }
    } else {
        rot13_file(stdin);
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

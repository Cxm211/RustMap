#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn toupper(_: std::os::raw::c_int) -> std::os::raw::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    fn strspn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
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
#[no_mangle]
pub static mut dih: [std::os::raw::c_char; 50] = [0; 50];
#[no_mangle]
pub static mut dah: [std::os::raw::c_char; 50] = [0; 50];
#[no_mangle]
pub static mut medium: [std::os::raw::c_char; 30] = [0; 30];
#[no_mangle]
pub static mut word: [std::os::raw::c_char; 30] = [0; 30];
#[no_mangle]
pub static mut dd: [*mut std::os::raw::c_char; 2] = unsafe {
    [dih.as_ptr() as *mut _, dah.as_ptr() as *mut _]
};
#[no_mangle]
pub static mut ascii: *const std::os::raw::c_char = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?'!/()&:;=+-_\"$@\0"
    as *const u8 as *const std::os::raw::c_char;
#[no_mangle]
pub static mut itu: [*const std::os::raw::c_char; 54] = [
    b"13\0" as *const u8 as *const std::os::raw::c_char,
    b"3111\0" as *const u8 as *const std::os::raw::c_char,
    b"3131\0" as *const u8 as *const std::os::raw::c_char,
    b"311\0" as *const u8 as *const std::os::raw::c_char,
    b"1\0" as *const u8 as *const std::os::raw::c_char,
    b"1131\0" as *const u8 as *const std::os::raw::c_char,
    b"331\0" as *const u8 as *const std::os::raw::c_char,
    b"1111\0" as *const u8 as *const std::os::raw::c_char,
    b"11\0" as *const u8 as *const std::os::raw::c_char,
    b"1333\0" as *const u8 as *const std::os::raw::c_char,
    b"313\0" as *const u8 as *const std::os::raw::c_char,
    b"1311\0" as *const u8 as *const std::os::raw::c_char,
    b"33\0" as *const u8 as *const std::os::raw::c_char,
    b"31\0" as *const u8 as *const std::os::raw::c_char,
    b"333\0" as *const u8 as *const std::os::raw::c_char,
    b"1331\0" as *const u8 as *const std::os::raw::c_char,
    b"3313\0" as *const u8 as *const std::os::raw::c_char,
    b"131\0" as *const u8 as *const std::os::raw::c_char,
    b"111\0" as *const u8 as *const std::os::raw::c_char,
    b"3\0" as *const u8 as *const std::os::raw::c_char,
    b"113\0" as *const u8 as *const std::os::raw::c_char,
    b"1113\0" as *const u8 as *const std::os::raw::c_char,
    b"133\0" as *const u8 as *const std::os::raw::c_char,
    b"3113\0" as *const u8 as *const std::os::raw::c_char,
    b"3133\0" as *const u8 as *const std::os::raw::c_char,
    b"3311\0" as *const u8 as *const std::os::raw::c_char,
    b"33333\0" as *const u8 as *const std::os::raw::c_char,
    b"13333\0" as *const u8 as *const std::os::raw::c_char,
    b"11333\0" as *const u8 as *const std::os::raw::c_char,
    b"11133\0" as *const u8 as *const std::os::raw::c_char,
    b"11113\0" as *const u8 as *const std::os::raw::c_char,
    b"11111\0" as *const u8 as *const std::os::raw::c_char,
    b"31111\0" as *const u8 as *const std::os::raw::c_char,
    b"33111\0" as *const u8 as *const std::os::raw::c_char,
    b"33311\0" as *const u8 as *const std::os::raw::c_char,
    b"33331\0" as *const u8 as *const std::os::raw::c_char,
    b"131313\0" as *const u8 as *const std::os::raw::c_char,
    b"331133\0" as *const u8 as *const std::os::raw::c_char,
    b"113311\0" as *const u8 as *const std::os::raw::c_char,
    b"133331\0" as *const u8 as *const std::os::raw::c_char,
    b"313133\0" as *const u8 as *const std::os::raw::c_char,
    b"31131\0" as *const u8 as *const std::os::raw::c_char,
    b"31331\0" as *const u8 as *const std::os::raw::c_char,
    b"313313\0" as *const u8 as *const std::os::raw::c_char,
    b"13111\0" as *const u8 as *const std::os::raw::c_char,
    b"333111\0" as *const u8 as *const std::os::raw::c_char,
    b"313131\0" as *const u8 as *const std::os::raw::c_char,
    b"31113\0" as *const u8 as *const std::os::raw::c_char,
    b"13131\0" as *const u8 as *const std::os::raw::c_char,
    b"311113\0" as *const u8 as *const std::os::raw::c_char,
    b"113313\0" as *const u8 as *const std::os::raw::c_char,
    b"131131\0" as *const u8 as *const std::os::raw::c_char,
    b"1113113\0" as *const u8 as *const std::os::raw::c_char,
    b"133131\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn append(
    mut s: *mut std::os::raw::c_char,
    mut morse: *const std::os::raw::c_char,
) {
    while *morse != 0 {
        strcat(s, dd[('3' as i32 == *morse as std::os::raw::c_int) as std::os::raw::c_int as usize]);
        morse = morse.offset(1);
        morse;
    }
    strcat(s, medium.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn translate(
    mut i: *const std::os::raw::c_char,
    mut o: *mut std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut pc: *const std::os::raw::c_char = core::ptr::null();
    sprintf(o, b"beep\0" as *const u8 as *const std::os::raw::c_char);
    while *i != 0 {
        pc = strchr(ascii, toupper(*i as std::os::raw::c_int));
        if pc.is_null() {
            strcat(o, word.as_mut_ptr());
        } else {
            append(o, itu[pc.offset_from(ascii) as std::os::raw::c_long as usize]);
        }
        i = i.offset(1);
        i;
    }
    strcat(o, word.as_mut_ptr());
    return o;
}
unsafe fn main_0(mut ac: std::os::raw::c_int, mut av: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut sin: [std::os::raw::c_char; 73] = [0; 73];
    let mut sout: [std::os::raw::c_char; 100000] = [0; 100000];
    let mut dit: std::os::raw::c_int = 100 as std::os::raw::c_int;
    if (1 as std::os::raw::c_int) < ac {
        if strlen(*av.offset(1 as std::os::raw::c_int as isize))
            != strspn(
                *av.offset(1 as std::os::raw::c_int as isize),
                b"0123456789\0" as *const u8 as *const std::os::raw::c_char,
            )
        {
            return 0 as std::os::raw::c_int
                * fprintf(
                    stderr,
                    b"use: %s [duration]   dit in ms, default %d\n\0" as *const u8
                        as *const std::os::raw::c_char,
                    *av,
                    dit,
                );
        }
        dit = if (1 as std::os::raw::c_int) < atoi(*av.offset(1 as std::os::raw::c_int as isize)) {
            if atoi(*av.offset(1 as std::os::raw::c_int as isize)) < 1000 as std::os::raw::c_int {
                atoi(*av.offset(1 as std::os::raw::c_int as isize))
            } else {
                1000 as std::os::raw::c_int
            }
        } else {
            1 as std::os::raw::c_int
        };
    }
    sprintf(
        dah.as_mut_ptr(),
        b" -n -f 440 -l %d -D %d\0" as *const u8 as *const std::os::raw::c_char,
        3 as std::os::raw::c_int * dit,
        dit,
    );
    sprintf(
        dih.as_mut_ptr(),
        b" -n -f 440 -l %d -D %d\0" as *const u8 as *const std::os::raw::c_char,
        dit,
        dit,
    );
    sprintf(
        medium.as_mut_ptr(),
        b" -n -D %d\0" as *const u8 as *const std::os::raw::c_char,
        (3 as std::os::raw::c_int - 1 as std::os::raw::c_int) * dit,
    );
    sprintf(
        word.as_mut_ptr(),
        b" -n -D %d\0" as *const u8 as *const std::os::raw::c_char,
        (7 as std::os::raw::c_int - (3 as std::os::raw::c_int - 1 as std::os::raw::c_int) - 1 as std::os::raw::c_int)
            * dit,
    );
    while !(fgets(sin.as_mut_ptr(), 72 as std::os::raw::c_int, stdin)).is_null() {
        puts(translate(sin.as_mut_ptr(), sout.as_mut_ptr()));
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

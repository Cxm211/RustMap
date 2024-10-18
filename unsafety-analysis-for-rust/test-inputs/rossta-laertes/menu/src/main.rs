#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
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
    let mut items: [*const std::os::raw::c_char; 5] = [
        b"fee fie\0" as *const u8 as *const std::os::raw::c_char,
        b"huff and puff\0" as *const u8 as *const std::os::raw::c_char,
        b"mirror mirror\0" as *const u8 as *const std::os::raw::c_char,
        b"tick tock\0" as *const u8 as *const std::os::raw::c_char,
        core::ptr::null(),
    ];
    let mut prompt: *const std::os::raw::c_char = b"Which is from the three pigs?\0" as *const u8
        as *const std::os::raw::c_char;
    printf(
        b"You chose %s.\n\0" as *const u8 as *const std::os::raw::c_char,
        menu_select(items.as_mut_ptr(), prompt),
    );
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn menu_select(
    mut items: *const *const std::os::raw::c_char,
    mut prompt: *const std::os::raw::c_char,
) -> *const std::os::raw::c_char {
    let mut buf: [std::os::raw::c_char; 8192] = [0; 8192];
    let mut i: std::os::raw::c_int = 0;
    let mut choice: std::os::raw::c_int = 0;
    let mut choice_max: std::os::raw::c_int = 0;
    if items.is_null() {
        return 0 as *const std::os::raw::c_char;
    }
    loop {
        i = 0 as std::os::raw::c_int;
        while !(*items.offset(i as isize)).is_null() {
            printf(
                b"%d) %s\n\0" as *const u8 as *const std::os::raw::c_char,
                i + 1 as std::os::raw::c_int,
                *items.offset(i as isize),
            );
            i += 1;
            i;
        }
        choice_max = i;
        if !prompt.is_null() {
            printf(b"%s \0" as *const u8 as *const std::os::raw::c_char, prompt);
        } else {
            printf(b"Choice? \0" as *const u8 as *const std::os::raw::c_char);
        }
        if !(fgets(
            buf.as_mut_ptr(),
            std::mem::size_of::<[std::os::raw::c_char; 8192]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
            stdin,
        ))
            .is_null()
        {
            choice = atoi(buf.as_mut_ptr());
        }
        if !(1 as std::os::raw::c_int > choice || choice > choice_max) {
            break;
        }
    }
    return *items.offset((choice - 1 as std::os::raw::c_int) as isize);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

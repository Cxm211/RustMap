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
    fn getline(
        __lineptr: *mut *mut std::os::raw::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn free(_: *mut u8);
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
pub type __ssize_t = std::os::raw::c_long;
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
pub type ssize_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut question: *mut std::os::raw::c_char = core::ptr::null_mut();
    let mut len: size_t = 0 as std::os::raw::c_int as size_t;
    let mut read: ssize_t = 0;
    let mut answers: [*const std::os::raw::c_char; 20] = [
        b"It is certain\0" as *const u8 as *const std::os::raw::c_char,
        b"It is decidedly so\0" as *const u8 as *const std::os::raw::c_char,
        b"Without a doubt\0" as *const u8 as *const std::os::raw::c_char,
        b"Yes, definitely\0" as *const u8 as *const std::os::raw::c_char,
        b"You may rely on it\0" as *const u8 as *const std::os::raw::c_char,
        b"As I see it, yes\0" as *const u8 as *const std::os::raw::c_char,
        b"Most likely\0" as *const u8 as *const std::os::raw::c_char,
        b"Outlook good\0" as *const u8 as *const std::os::raw::c_char,
        b"Signs point to yes\0" as *const u8 as *const std::os::raw::c_char,
        b"Yes\0" as *const u8 as *const std::os::raw::c_char,
        b"Reply hazy, try again\0" as *const u8 as *const std::os::raw::c_char,
        b"Ask again later\0" as *const u8 as *const std::os::raw::c_char,
        b"Better not tell you now\0" as *const u8 as *const std::os::raw::c_char,
        b"Cannot predict now\0" as *const u8 as *const std::os::raw::c_char,
        b"Concentrate and ask again\0" as *const u8 as *const std::os::raw::c_char,
        b"Don't bet on it\0" as *const u8 as *const std::os::raw::c_char,
        b"My reply is no\0" as *const u8 as *const std::os::raw::c_char,
        b"My sources say no\0" as *const u8 as *const std::os::raw::c_char,
        b"Outlook not so good\0" as *const u8 as *const std::os::raw::c_char,
        b"Very doubtful\0" as *const u8 as *const std::os::raw::c_char,
    ];
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    printf(
        b"Please enter your question or a blank line to quit.\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    loop {
        printf(b"\n? : \0" as *const u8 as *const std::os::raw::c_char);
        read = getline(&mut question, &mut len, stdin);
        if read < 2 as std::os::raw::c_int as std::os::raw::c_long {
            break;
        }
        printf(
            b"\n%s\n\0" as *const u8 as *const std::os::raw::c_char,
            answers[(rand() % 20 as std::os::raw::c_int) as usize],
        );
    }
    if !question.is_null() {
        free(question as *mut u8);
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

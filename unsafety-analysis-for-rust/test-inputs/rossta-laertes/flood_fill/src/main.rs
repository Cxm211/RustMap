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
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fscanf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fgetc(__stream: *mut FILE) -> std::os::raw::c_int;
    fn ungetc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    fn feof(__stream: *mut FILE) -> std::os::raw::c_int;
    fn ferror(__stream: *mut FILE) -> std::os::raw::c_int;
    fn exit(_: std::os::raw::c_int) -> !;
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
static mut width: std::os::raw::c_int = 0;
static mut height: std::os::raw::c_int = 0;
static mut bitmap: [[std::os::raw::c_uchar; 2048]; 2048] = [[0; 2048]; 2048];
static mut oldColor: std::os::raw::c_uchar = 0;
static mut newColor: std::os::raw::c_uchar = 0;
#[no_mangle]
pub unsafe extern "C" fn floodFill(mut i: std::os::raw::c_int, mut j: std::os::raw::c_int) {
    if 0 as std::os::raw::c_int <= i && i < height && 0 as std::os::raw::c_int <= j && j < width
        && bitmap[i as usize][j as usize] as std::os::raw::c_int == oldColor as std::os::raw::c_int
    {
        bitmap[i as usize][j as usize] = newColor;
        floodFill(i - 1 as std::os::raw::c_int, j);
        floodFill(i + 1 as std::os::raw::c_int, j);
        floodFill(i, j - 1 as std::os::raw::c_int);
        floodFill(i, j + 1 as std::os::raw::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn skipLine(mut file: *mut FILE) {
    while ferror(file) == 0 && feof(file) == 0 && fgetc(file) != '\n' as i32 {}
}
#[no_mangle]
pub unsafe extern "C" fn skipCommentLines(mut file: *mut FILE) {
    let mut c: std::os::raw::c_int = 0;
    let mut comment: std::os::raw::c_int = '#' as i32;
    loop {
        c = fgetc(file);
        if !(c == comment) {
            break;
        }
        skipLine(file);
    }
    ungetc(c, file);
}
#[no_mangle]
pub unsafe extern "C" fn readPortableBitMap(mut file: *mut FILE) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    skipLine(file);
    skipCommentLines(file);
    fscanf(
        file,
        b"%d\0" as *const u8 as *const std::os::raw::c_char,
        &mut width as *mut std::os::raw::c_int,
    );
    skipCommentLines(file);
    fscanf(
        file,
        b"%d\0" as *const u8 as *const std::os::raw::c_char,
        &mut height as *mut std::os::raw::c_int,
    );
    skipCommentLines(file);
    if width <= 2048 as std::os::raw::c_int && height <= 2048 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int;
        while i < height {
            j = 0 as std::os::raw::c_int;
            while j < width {
                fscanf(
                    file,
                    b"%1d\0" as *const u8 as *const std::os::raw::c_char,
                    &mut *(*bitmap.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(j as isize) as *mut std::os::raw::c_uchar,
                );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        exit(1 as std::os::raw::c_int);
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn writePortableBitMap(mut file: *mut FILE) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    fprintf(file, b"P1\n\0" as *const u8 as *const std::os::raw::c_char);
    fprintf(file, b"%d %d\n\0" as *const u8 as *const std::os::raw::c_char, width, height);
    i = 0 as std::os::raw::c_int;
    while i < height {
        j = 0 as std::os::raw::c_int;
        while j < width {
            fprintf(
                file,
                b"%1d\0" as *const u8 as *const std::os::raw::c_char,
                bitmap[i as usize][j as usize] as std::os::raw::c_int,
            );
            j += 1;
            j;
        }
        fprintf(file, b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    oldColor = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    newColor = (if oldColor as std::os::raw::c_int != 0 {
        0 as std::os::raw::c_int
    } else {
        1 as std::os::raw::c_int
    }) as std::os::raw::c_uchar;
    readPortableBitMap(stdin);
    floodFill(height / 2 as std::os::raw::c_int, width / 2 as std::os::raw::c_int);
    writePortableBitMap(stdout);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

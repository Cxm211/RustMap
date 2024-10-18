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
    fn perror(__s: *const std::os::raw::c_char);
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn getc(__stream: *mut FILE) -> std::os::raw::c_int;
    fn exit(_: std::os::raw::c_int) -> !;
    fn log10f(_: std::os::raw::c_float) -> std::os::raw::c_float;
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
pub unsafe extern "C" fn benford_distribution() -> *mut std::os::raw::c_float {
    static mut prob: [std::os::raw::c_float; 9] = [0.; 9];
    let mut i: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        prob[(i - 1 as std::os::raw::c_int)
            as usize] = log10f(
            (1 as std::os::raw::c_int as std::os::raw::c_double + 1.0f64 / i as std::os::raw::c_double)
                as std::os::raw::c_float,
        );
        i += 1;
        i;
    }
    return prob.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn get_actual_distribution(
    mut fn_0: *mut std::os::raw::c_char,
) -> *mut std::os::raw::c_float {
    let mut input: *mut FILE = fopen(fn_0, b"r\0" as *const u8 as *const std::os::raw::c_char);
    if input.is_null() {
        perror(b"Can't open file\0" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    let mut tally: [std::os::raw::c_int; 9] = [0 as std::os::raw::c_int, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut c: std::os::raw::c_char = 0;
    let mut total: std::os::raw::c_int = 0 as std::os::raw::c_int;
    loop {
        c = getc(input) as std::os::raw::c_char;
        if !(c as std::os::raw::c_int != -(1 as std::os::raw::c_int)) {
            break;
        }
        while (c as std::os::raw::c_int) < '1' as i32 || c as std::os::raw::c_int > '9' as i32 {
            c = getc(input) as std::os::raw::c_char;
        }
        tally[(c as std::os::raw::c_int - '1' as i32) as usize] += 1;
        tally[(c as std::os::raw::c_int - '1' as i32) as usize];
        total += 1;
        total;
        loop {
            c = getc(input) as std::os::raw::c_char;
            if !(c as std::os::raw::c_int != '\n' as i32
                && c as std::os::raw::c_int != -(1 as std::os::raw::c_int))
            {
                break;
            }
        }
    }
    fclose(input);
    static mut freq: [std::os::raw::c_float; 9] = [0.; 9];
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < 9 as std::os::raw::c_int {
        freq[i as usize] = tally[i as usize] as std::os::raw::c_float / total as std::os::raw::c_float;
        i += 1;
        i;
    }
    return freq.as_mut_ptr();
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    if argc != 2 as std::os::raw::c_int {
        printf(b"Usage: benford <file>\n\0" as *const u8 as *const std::os::raw::c_char);
        return 1 as std::os::raw::c_int;
    }
    let mut actual: *mut std::os::raw::c_float = get_actual_distribution(
        *argv.offset(1 as std::os::raw::c_int as isize),
    );
    let mut expected: *mut std::os::raw::c_float = benford_distribution();
    puts(b"digit\tactual\texpected\0" as *const u8 as *const std::os::raw::c_char);
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < 9 as std::os::raw::c_int {
        printf(
            b"%d\t%.3f\t%.3f\n\0" as *const u8 as *const std::os::raw::c_char,
            i + 1 as std::os::raw::c_int,
            *actual.offset(i as isize) as std::os::raw::c_double,
            *expected.offset(i as isize) as std::os::raw::c_double,
        );
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

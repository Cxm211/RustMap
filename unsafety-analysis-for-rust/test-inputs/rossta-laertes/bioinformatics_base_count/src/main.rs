#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn free(_: *mut u8);
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genome {
    pub strand: * mut std::os::raw::c_char,
    pub length: std::os::raw::c_int,
    pub next: * mut crate::genome,
}
impl std::default::Default for genome {
    fn default() -> Self {
        genome {
        strand: 0 as * mut std::os::raw::c_char,
        length: std::os::raw::c_int::default(),
        next: core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub static mut genomeData: *mut genome = 0 as *const genome as *mut genome;
#[no_mangle]
pub static mut totalLength: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut Adenine: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut Cytosine: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut Guanine: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut Thymine: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub extern "C" fn numDigits(mut num: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut len: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while num > 10 as std::os::raw::c_int {
        num = num / 10 as std::os::raw::c_int;
        len += 1;
        len;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn buildGenome(mut str: *mut std::os::raw::c_char) {
    let mut len: std::os::raw::c_int = strlen(str as *const std::os::raw::c_char) as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut genomeIterator: *mut genome = core::ptr::null_mut();
    let mut newGenome: *mut genome = core::ptr::null_mut();
    totalLength += len;
    i = 0 as std::os::raw::c_int;
    while i < len {
        match *str.offset(i as isize) as std::os::raw::c_int {
            65 => {
                Adenine += 1;
                Adenine;
            }
            84 => {
                Thymine += 1;
                Thymine;
            }
            67 => {
                Cytosine += 1;
                Cytosine;
            }
            71 => {
                Guanine += 1;
                Guanine;
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if genomeData.is_null() {
        genomeData = malloc(std::mem::size_of::<genome>() as std::os::raw::c_ulong)
            as *mut genome;
        (*genomeData)
            .strand = malloc(
            (len as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_char;
        strcpy((*genomeData).strand, str as *const std::os::raw::c_char);
        (*genomeData).length = len;
        (*genomeData).next = core::ptr::null_mut();
    } else {
        genomeIterator = genomeData;
        while !((*genomeIterator).next).is_null() {
            genomeIterator = (*genomeIterator).next;
        }
        newGenome = malloc(std::mem::size_of::<genome>() as std::os::raw::c_ulong)
            as *mut genome;
        (*newGenome)
            .strand = malloc(
            (len as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_char;
        strcpy((*newGenome).strand, str as *const std::os::raw::c_char);
        (*newGenome).length = len;
        (*newGenome).next = core::ptr::null_mut();
        (*genomeIterator).next = newGenome;
    };
}
#[no_mangle]
pub unsafe extern "C" fn printGenome() {
    let mut genomeIterator: *mut genome = genomeData;
    let mut width: std::os::raw::c_int = numDigits(totalLength);
    let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    printf(b"Sequence:\n\0" as *const u8 as *const std::os::raw::c_char);
    while !genomeIterator.is_null() {
        printf(
            b"\n%*d%3s%3s\0" as *const u8 as *const std::os::raw::c_char,
            width + 1 as std::os::raw::c_int,
            len,
            b":\0" as *const u8 as *const std::os::raw::c_char,
            (*genomeIterator).strand,
        );
        len += (*genomeIterator).length;
        genomeIterator = (*genomeIterator).next;
    }
    printf(b"\n\nBase Count\n----------\n\n\0" as *const u8 as *const std::os::raw::c_char);
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const std::os::raw::c_char,
        'A' as i32,
        b":\0" as *const u8 as *const std::os::raw::c_char,
        width + 1 as std::os::raw::c_int,
        Adenine,
    );
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const std::os::raw::c_char,
        'T' as i32,
        b":\0" as *const u8 as *const std::os::raw::c_char,
        width + 1 as std::os::raw::c_int,
        Thymine,
    );
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const std::os::raw::c_char,
        'C' as i32,
        b":\0" as *const u8 as *const std::os::raw::c_char,
        width + 1 as std::os::raw::c_int,
        Cytosine,
    );
    printf(
        b"%3c%3s%*d\n\0" as *const u8 as *const std::os::raw::c_char,
        'G' as i32,
        b":\0" as *const u8 as *const std::os::raw::c_char,
        width + 1 as std::os::raw::c_int,
        Guanine,
    );
    printf(
        b"\n%3s%*d\n\0" as *const u8 as *const std::os::raw::c_char,
        b"Total:\0" as *const u8 as *const std::os::raw::c_char,
        width + 1 as std::os::raw::c_int,
        Adenine + Thymine + Cytosine + Guanine,
    );
    free(genomeData as *mut u8);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut str: [std::os::raw::c_char; 100] = [0; 100];
    let mut counter: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut len: std::os::raw::c_int = 0;
    if argc != 2 as std::os::raw::c_int {
        printf(
            b"Usage : %s <Gene file name>\n\0" as *const u8 as *const std::os::raw::c_char,
            *argv.offset(0 as std::os::raw::c_int as isize),
        );
        return 0 as std::os::raw::c_int;
    }
    let mut fp: *mut FILE = fopen(
        *argv.offset(1 as std::os::raw::c_int as isize),
        b"r\0" as *const u8 as *const std::os::raw::c_char,
    );
    while fscanf(fp, b"%s\0" as *const u8 as *const std::os::raw::c_char, str.as_mut_ptr())
        != -(1 as std::os::raw::c_int)
    {
        buildGenome(str.as_mut_ptr());
    }
    fclose(fp);
    printGenome();
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

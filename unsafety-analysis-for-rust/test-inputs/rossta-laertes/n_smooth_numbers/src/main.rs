#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type uint32_t = std::os::raw::c_uint;
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
pub unsafe extern "C" fn xmalloc(mut n: size_t) -> *mut u8 {
    let mut ptr: *mut u8 = malloc(n);
    if ptr.is_null() {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut p: *mut u8,
    mut n: size_t,
) -> *mut u8 {
    let mut ptr: *mut u8 = realloc(p, n);
    if ptr.is_null() {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    return ptr;
}
#[no_mangle]
pub extern "C" fn is_prime(mut n: uint32_t) -> bool {
    if n == 2 as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int != 0;
    }
    if n < 2 as std::os::raw::c_int as std::os::raw::c_uint
        || n.wrapping_rem(2 as std::os::raw::c_int as std::os::raw::c_uint)
            == 0 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return 0 as std::os::raw::c_int != 0;
    }
    let mut p: uint32_t = 3 as std::os::raw::c_int as uint32_t;
    while p.wrapping_mul(p) <= n {
        if n.wrapping_rem(p) == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as std::os::raw::c_int != 0;
        }
        p = (p as std::os::raw::c_uint).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
            as uint32_t as uint32_t;
    }
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn find_primes<'a1>(
    mut from: std::os::raw::c_uint,
    mut to: std::os::raw::c_uint,
    mut primes: Option<&'a1 mut * mut std::os::raw::c_uint>,
) -> std::os::raw::c_uint {
    let mut count: uint32_t = 0 as std::os::raw::c_int as uint32_t;
    let mut buffer_length: uint32_t = 16 as std::os::raw::c_int as uint32_t;
    let mut buffer: *mut uint32_t = xmalloc(
        (std::mem::size_of::<uint32_t>() as std::os::raw::c_ulong)
            .wrapping_mul(buffer_length as std::os::raw::c_ulong),
    ) as *mut uint32_t;
    let mut p: uint32_t = from;
    while p <= to {
        if is_prime(p) {
            if count >= buffer_length {
                let mut new_length: uint32_t = buffer_length
                    .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_uint);
                if new_length < count.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) {
                    new_length = count.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
                }
                buffer = xrealloc(
                    buffer as *mut u8,
                    (std::mem::size_of::<uint32_t>() as std::os::raw::c_ulong)
                        .wrapping_mul(new_length as std::os::raw::c_ulong),
                ) as *mut uint32_t;
                buffer_length = new_length;
            }
            let fresh0 = count;
            count = count.wrapping_add(1);
            *buffer.offset(fresh0 as isize) = p;
        }
        p = p.wrapping_add(1);
        p;
    }
    *borrow_mut(&mut primes).unwrap() = buffer;
    return count;
}
#[no_mangle]
pub extern "C" fn free_numbers<'a1>(mut numbers: Option<&'a1 mut std::os::raw::c_int>, mut count: std::os::raw::c_ulong) {}
#[no_mangle]
pub unsafe extern "C" fn print_nsmooth_numbers(
    mut n: uint32_t,
    mut begin: uint32_t,
    mut count: uint32_t,
) {
    let mut num: uint32_t = begin.wrapping_add(count);
    printf(b"%u: \0" as *const u8 as *const std::os::raw::c_char, n);
    let mut i: uint32_t = 1 as std::os::raw::c_int as uint32_t;
    while i < count {
        printf(b", \0" as *const u8 as *const std::os::raw::c_char);
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"First 25 n-smooth numbers for n = 2 -> 29:\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    let mut n: uint32_t = 2 as std::os::raw::c_int as uint32_t;
    while n <= 29 as std::os::raw::c_int as std::os::raw::c_uint {
        if is_prime(n) {
            print_nsmooth_numbers(
                n,
                0 as std::os::raw::c_int as uint32_t,
                25 as std::os::raw::c_int as uint32_t,
            );
        }
        n = n.wrapping_add(1);
        n;
    }
    printf(
        b"\n3 n-smooth numbers starting from 3000th for n = 3 -> 29:\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    let mut n_0: uint32_t = 3 as std::os::raw::c_int as uint32_t;
    while n_0 <= 29 as std::os::raw::c_int as std::os::raw::c_uint {
        if is_prime(n_0) {
            print_nsmooth_numbers(
                n_0,
                2999 as std::os::raw::c_int as uint32_t,
                3 as std::os::raw::c_int as uint32_t,
            );
        }
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    printf(
        b"\n20 n-smooth numbers starting from 30,000th for n = 503 -> 521:\n\0"
            as *const u8 as *const std::os::raw::c_char,
    );
    let mut n_1: uint32_t = 503 as std::os::raw::c_int as uint32_t;
    while n_1 <= 521 as std::os::raw::c_int as std::os::raw::c_uint {
        if is_prime(n_1) {
            print_nsmooth_numbers(
                n_1,
                29999 as std::os::raw::c_int as uint32_t,
                20 as std::os::raw::c_int as uint32_t,
            );
        }
        n_1 = n_1.wrapping_add(1);
        n_1;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

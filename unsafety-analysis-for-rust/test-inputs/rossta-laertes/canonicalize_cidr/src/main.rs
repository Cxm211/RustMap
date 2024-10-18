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
    fn snprintf(
        _: *mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint32_t = std::os::raw::c_uint;
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
pub type uint32_t = std::os::raw::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cidr_tag {
    pub address: std::os::raw::c_uint,
    pub mask_length: std::os::raw::c_uint,
}
impl std::default::Default for cidr_tag {
    fn default() -> Self {
        cidr_tag {
        address: std::os::raw::c_uint::default(),
        mask_length: std::os::raw::c_uint::default()
        }
    }
}

pub type cidr_t = crate::cidr_tag;
#[no_mangle]
pub unsafe extern "C" fn cidr_parse<'a1>(
    mut str: * const std::os::raw::c_char,
    mut cidr: Option<&'a1 mut crate::cidr_tag>,
) -> bool {
    let mut a: std::os::raw::c_int = 0;
    let mut b: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = 0;
    let mut m: std::os::raw::c_int = 0;
    if sscanf(
        str,
        b"%d.%d.%d.%d/%d\0" as *const u8 as *const std::os::raw::c_char,
        &mut a as *mut std::os::raw::c_int,
        &mut b as *mut std::os::raw::c_int,
        &mut c as *mut std::os::raw::c_int,
        &mut d as *mut std::os::raw::c_int,
        &mut m as *mut std::os::raw::c_int,
    ) != 5 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int != 0;
    }
    if m < 1 as std::os::raw::c_int || m > 32 as std::os::raw::c_int || a < 0 as std::os::raw::c_int
        || a > 255 as std::os::raw::c_int || b < 0 as std::os::raw::c_int || b > 255 as std::os::raw::c_int
        || c < 0 as std::os::raw::c_int || c > 255 as std::os::raw::c_int || d < 0 as std::os::raw::c_int
        || d > 255 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int != 0;
    }
    let mut mask: uint32_t = !(((1 as std::os::raw::c_int) << 32 as std::os::raw::c_int - m)
        - 1 as std::os::raw::c_int) as uint32_t;
    let mut address: uint32_t = ((a << 24 as std::os::raw::c_int) + (b << 16 as std::os::raw::c_int)
        + (c << 8 as std::os::raw::c_int) + d) as uint32_t;
    address &= mask;
    (*borrow_mut(&mut cidr).unwrap()).address = address;
    (*borrow_mut(&mut cidr).unwrap()).mask_length = m as std::os::raw::c_uint;
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cidr_format<'a1>(
    mut cidr: Option<&'a1 crate::cidr_tag>,
    mut str: * mut std::os::raw::c_char,
    mut size: std::os::raw::c_ulong,
) {
    let mut address: uint32_t = (*cidr.unwrap()).address;
    let mut d: std::os::raw::c_uint = address & 255 as std::os::raw::c_int as std::os::raw::c_uint;
    address >>= 8 as std::os::raw::c_int;
    let mut c: std::os::raw::c_uint = address & 255 as std::os::raw::c_int as std::os::raw::c_uint;
    address >>= 8 as std::os::raw::c_int;
    let mut b: std::os::raw::c_uint = address & 255 as std::os::raw::c_int as std::os::raw::c_uint;
    address >>= 8 as std::os::raw::c_int;
    let mut a: std::os::raw::c_uint = address & 255 as std::os::raw::c_int as std::os::raw::c_uint;
    snprintf(
        str,
        size,
        b"%u.%u.%u.%u/%u\0" as *const u8 as *const std::os::raw::c_char,
        a,
        b,
        c,
        d,
        (*cidr.unwrap()).mask_length,
    );
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut tests: [*const std::os::raw::c_char; 6] = [
        b"87.70.141.1/22\0" as *const u8 as *const std::os::raw::c_char,
        b"36.18.154.103/12\0" as *const u8 as *const std::os::raw::c_char,
        b"62.62.197.11/29\0" as *const u8 as *const std::os::raw::c_char,
        b"67.137.119.181/4\0" as *const u8 as *const std::os::raw::c_char,
        b"161.214.74.21/24\0" as *const u8 as *const std::os::raw::c_char,
        b"184.232.176.184/18\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (std::mem::size_of::<[*const std::os::raw::c_char; 6]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong)
    {
        let mut cidr: cidr_t = cidr_t {
            address: 0,
            mask_length: 0,
        };
        if cidr_parse(tests[i as usize], Some(&mut cidr)) {
            let mut out: [std::os::raw::c_char; 32] = [0; 32];
            cidr_format(
                borrow(& Some(&mut cidr)),
                out.as_mut_ptr(),
                std::mem::size_of::<[std::os::raw::c_char; 32]>() as std::os::raw::c_ulong,
            );
            printf(
                b"%-18s -> %s\n\0" as *const u8 as *const std::os::raw::c_char,
                tests[i as usize],
                out.as_mut_ptr(),
            );
        } else {
            fprintf(
                stderr,
                b"%s: invalid CIDR\n\0" as *const u8 as *const std::os::raw::c_char,
                tests[i as usize],
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

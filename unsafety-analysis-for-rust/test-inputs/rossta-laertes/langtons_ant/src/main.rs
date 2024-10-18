#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn memmove(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn usleep(__useconds: __useconds_t) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __useconds_t = std::os::raw::c_uint;
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
pub static mut w: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut h: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut pix: *mut std::os::raw::c_uchar = 0 as *const std::os::raw::c_uchar as *mut std::os::raw::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn refresh(mut x: std::os::raw::c_int, mut y: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    printf(b"\x1B[H\0" as *const u8 as *const std::os::raw::c_char);
    k = 0 as std::os::raw::c_int;
    i = k;
    while i < h {
        j = 0 as std::os::raw::c_int;
        while j < w {
            putchar(
                if *pix.offset(k as isize) as std::os::raw::c_int != 0 {
                    '#' as i32
                } else {
                    ' ' as i32
                },
            );
            j += 1;
            j;
            k += 1;
            k;
        }
        putchar('\n' as i32);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn walk() {
    let mut dx: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut dy: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut x: std::os::raw::c_int = w / 2 as std::os::raw::c_int;
    let mut y: std::os::raw::c_int = h / 2 as std::os::raw::c_int;
    pix = calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong, (w * h) as std::os::raw::c_ulong)
        as *mut std::os::raw::c_uchar;
    printf(b"\x1B[H\x1B[J\0" as *const u8 as *const std::os::raw::c_char);
    loop {
        i = y * w + x;
        if *pix.offset(i as isize) != 0 {
            k = dx;
            dx = -dy;
            dy = k;
        } else {
            k = dy;
            dy = -dx;
            dx = k;
        }
        *pix
            .offset(
                i as isize,
            ) = (*pix.offset(i as isize) == 0) as std::os::raw::c_int as std::os::raw::c_uchar;
        printf(
            b"\x1B[%d;%dH%c\0" as *const u8 as *const std::os::raw::c_char,
            y + 1 as std::os::raw::c_int,
            x + 1 as std::os::raw::c_int,
            if *pix.offset(i as isize) as std::os::raw::c_int != 0 {
                '#' as i32
            } else {
                ' ' as i32
            },
        );
        x += dx;
        y += dy;
        k = 0 as std::os::raw::c_int;
        if x < 0 as std::os::raw::c_int {
            memmove(
                pix.offset(1 as std::os::raw::c_int as isize) as *mut u8,
                pix as *const u8,
                (w * h - 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
            );
            i = 0 as std::os::raw::c_int;
            while i < w * h {
                *pix.offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                i += w;
            }
            x += 1;
            x;
            k = 1 as std::os::raw::c_int;
        } else if x >= w {
            memmove(
                pix as *mut u8,
                pix.offset(1 as std::os::raw::c_int as isize) as *const u8,
                (w * h - 1 as std::os::raw::c_int) as std::os::raw::c_ulong,
            );
            i = w - 1 as std::os::raw::c_int;
            while i < w * h {
                *pix.offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                i += w;
            }
            x -= 1;
            x;
            k = 1 as std::os::raw::c_int;
        }
        if y >= h {
            memmove(
                pix as *mut u8,
                pix.offset(w as isize) as *const u8,
                (w * (h - 1 as std::os::raw::c_int)) as std::os::raw::c_ulong,
            );
            memset(
                pix.offset((w * (h - 1 as std::os::raw::c_int)) as isize) as *mut u8,
                0 as std::os::raw::c_int,
                w as std::os::raw::c_ulong,
            );
            y -= 1;
            y;
            k = 1 as std::os::raw::c_int;
        } else if y < 0 as std::os::raw::c_int {
            memmove(
                pix.offset(w as isize) as *mut u8,
                pix as *const u8,
                (w * (h - 1 as std::os::raw::c_int)) as std::os::raw::c_ulong,
            );
            memset(pix as *mut u8, 0 as std::os::raw::c_int, w as std::os::raw::c_ulong);
            y += 1;
            y;
            k = 1 as std::os::raw::c_int;
        }
        if k != 0 {
            refresh(x, y);
        }
        printf(
            b"\x1B[%d;%dH\x1B[31m@\x1B[m\0" as *const u8 as *const std::os::raw::c_char,
            y + 1 as std::os::raw::c_int,
            x + 1 as std::os::raw::c_int,
        );
        fflush(stdout);
        usleep(10000 as std::os::raw::c_int as __useconds_t);
    };
}
unsafe fn main_0(mut c: std::os::raw::c_int, mut v: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    if c > 1 as std::os::raw::c_int {
        w = atoi(*v.offset(1 as std::os::raw::c_int as isize));
    }
    if c > 2 as std::os::raw::c_int {
        h = atoi(*v.offset(2 as std::os::raw::c_int as isize));
    }
    if w < 40 as std::os::raw::c_int {
        w = 40 as std::os::raw::c_int;
    }
    if h < 25 as std::os::raw::c_int {
        h = 25 as std::os::raw::c_int;
    }
    walk();
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

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fwrite(
        _: *const u8,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fmod(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
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
pub struct rgb {
    pub r: std::os::raw::c_double,
    pub g: std::os::raw::c_double,
    pub b: std::os::raw::c_double,
}
impl std::default::Default for rgb {
    fn default() -> Self {
        rgb {
        r: std::os::raw::c_double::default(),
        g: std::os::raw::c_double::default(),
        b: std::os::raw::c_double::default()
        }
    }
}

#[no_mangle]
pub static mut x: std::os::raw::c_longlong = 0;
#[no_mangle]
pub static mut y: std::os::raw::c_longlong = 0;
#[no_mangle]
pub static mut dx: std::os::raw::c_longlong = 0;
#[no_mangle]
pub static mut dy: std::os::raw::c_longlong = 0;
#[no_mangle]
pub static mut scale: std::os::raw::c_longlong = 0;
#[no_mangle]
pub static mut clen: std::os::raw::c_longlong = 0;
#[no_mangle]
pub static mut cscale: std::os::raw::c_longlong = 0;
#[no_mangle]
pub static mut pix: *mut *mut rgb = 0 as *const *mut rgb as *mut *mut rgb;
#[no_mangle]
pub unsafe extern "C" fn sc_up() {
    scale *= 2 as std::os::raw::c_int as std::os::raw::c_longlong;
    x *= 2 as std::os::raw::c_int as std::os::raw::c_longlong;
    y *= 2 as std::os::raw::c_int as std::os::raw::c_longlong;
    cscale *= 3 as std::os::raw::c_int as std::os::raw::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn h_rgb(mut x_0: std::os::raw::c_longlong, mut y_0: std::os::raw::c_longlong) {
    let mut p: _ = Some(&mut *(*pix.offset(y_0 as isize)).offset(x_0 as isize));
    let mut h: std::os::raw::c_double = 6.0f64 * clen as std::os::raw::c_double
        / cscale as std::os::raw::c_double;
    let mut VAL: std::os::raw::c_double = 1 as std::os::raw::c_int as std::os::raw::c_double;
    let mut c: std::os::raw::c_double = 1 as std::os::raw::c_int as std::os::raw::c_double * VAL;
    let mut X: std::os::raw::c_double = c
        * (1 as std::os::raw::c_int as std::os::raw::c_double
            - fabs(
                fmod(h, 2 as std::os::raw::c_int as std::os::raw::c_double)
                    - 1 as std::os::raw::c_int as std::os::raw::c_double,
            ));
    match h as std::os::raw::c_int {
        0 => {
            (*borrow_mut(&mut p).unwrap()).r += c;
            (*borrow_mut(&mut p).unwrap()).g += X;
            return;
        }
        1 => {
            (*borrow_mut(&mut p).unwrap()).r += X;
            (*borrow_mut(&mut p).unwrap()).g += c;
            return;
        }
        2 => {
            (*borrow_mut(&mut p).unwrap()).g += c;
            (*borrow_mut(&mut p).unwrap()).b += X;
            return;
        }
        3 => {
            (*borrow_mut(&mut p).unwrap()).g += X;
            (*borrow_mut(&mut p).unwrap()).b += c;
            return;
        }
        4 => {
            (*borrow_mut(&mut p).unwrap()).r += X;
            (*borrow_mut(&mut p).unwrap()).b += c;
            return;
        }
        _ => {
            (*borrow_mut(&mut p).unwrap()).r += c;
            (*borrow_mut(&mut p).unwrap()).b += X;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn iter_string(mut str: *const std::os::raw::c_char, mut d: std::os::raw::c_int) {
    let mut len: std::os::raw::c_longlong = 0;
    while *str as std::os::raw::c_int != '\0' as i32 {
        let fresh0 = str;
        str = str.offset(1);
        match *fresh0 as std::os::raw::c_int {
            88 => {
                if d != 0 {
                    iter_string(
                        b"XHXVX\0" as *const u8 as *const std::os::raw::c_char,
                        d - 1 as std::os::raw::c_int,
                    );
                } else {
                    clen += 1;
                    clen;
                    h_rgb(x / scale, y / scale);
                    x += dx;
                    y -= dy;
                }
            }
            86 => {
                len = ((1 as std::os::raw::c_ulonglong) << d) as std::os::raw::c_longlong;
                loop {
                    let fresh1 = len;
                    len = len - 1;
                    if !(fresh1 != 0) {
                        break;
                    }
                    clen += 1;
                    clen;
                    h_rgb(x / scale, y / scale);
                    y += dy;
                }
            }
            72 => {
                len = ((1 as std::os::raw::c_ulonglong) << d) as std::os::raw::c_longlong;
                loop {
                    let fresh2 = len;
                    len = len - 1;
                    if !(fresh2 != 0) {
                        break;
                    }
                    clen += 1;
                    clen;
                    h_rgb(x / scale, y / scale);
                    x -= dx;
                }
            }
            _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sierp(mut leng: std::os::raw::c_long, mut depth: std::os::raw::c_int) {
    let mut i: std::os::raw::c_long = 0;
    let mut h: std::os::raw::c_long = leng + 20 as std::os::raw::c_int as std::os::raw::c_long;
    let mut w: std::os::raw::c_long = leng + 20 as std::os::raw::c_int as std::os::raw::c_long;
    let mut buf: *mut rgb = malloc(
        (std::mem::size_of::<rgb>() as std::os::raw::c_ulong)
            .wrapping_mul(w as std::os::raw::c_ulong)
            .wrapping_mul(h as std::os::raw::c_ulong),
    ) as *mut rgb;
    pix = malloc(
        (std::mem::size_of::<*mut rgb>() as std::os::raw::c_ulong)
            .wrapping_mul(h as std::os::raw::c_ulong),
    ) as *mut *mut rgb;
    i = 0 as std::os::raw::c_int as std::os::raw::c_long;
    while i < h {
        let ref mut fresh3 = *pix.offset(i as isize);
        *fresh3 = buf.offset((w * i) as isize);
        i += 1;
        i;
    }
    memset(
        buf as *mut u8,
        0 as std::os::raw::c_int,
        (std::mem::size_of::<rgb>() as std::os::raw::c_ulong)
            .wrapping_mul(w as std::os::raw::c_ulong)
            .wrapping_mul(h as std::os::raw::c_ulong),
    );
    y = 10 as std::os::raw::c_int as std::os::raw::c_longlong;
    x = y;
    dx = leng as std::os::raw::c_longlong;
    dy = leng as std::os::raw::c_longlong;
    scale = 1 as std::os::raw::c_int as std::os::raw::c_longlong;
    clen = 0 as std::os::raw::c_int as std::os::raw::c_longlong;
    cscale = 3 as std::os::raw::c_int as std::os::raw::c_longlong;
    i = 0 as std::os::raw::c_int as std::os::raw::c_long;
    while i < depth as std::os::raw::c_long {
        sc_up();
        i += 1;
        i;
    }
    iter_string(b"VXH\0" as *const u8 as *const std::os::raw::c_char, depth);
    let mut fpix: *mut std::os::raw::c_uchar = malloc(
        (w * h * 3 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_uchar;
    let mut maxv: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut dbuf: *mut std::os::raw::c_double = buf as *mut std::os::raw::c_double;
    i = 3 as std::os::raw::c_int as std::os::raw::c_long * w * h - 1 as std::os::raw::c_int as std::os::raw::c_long;
    while i >= 0 as std::os::raw::c_int as std::os::raw::c_long {
        if *dbuf.offset(i as isize) > maxv {
            maxv = *dbuf.offset(i as isize);
        }
        i -= 1;
        i;
    }
    i = 3 as std::os::raw::c_int as std::os::raw::c_long * h * w - 1 as std::os::raw::c_int as std::os::raw::c_long;
    while i >= 0 as std::os::raw::c_int as std::os::raw::c_long {
        *fpix
            .offset(
                i as isize,
            ) = (255 as std::os::raw::c_int as std::os::raw::c_double * *dbuf.offset(i as isize) / maxv)
            as std::os::raw::c_uchar;
        i -= 1;
        i;
    }
    printf(b"P6\n%ld %ld\n255\n\0" as *const u8 as *const std::os::raw::c_char, w, h);
    fflush(stdout);
    fwrite(
        fpix as *const u8,
        (h * w * 3 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_ulong,
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        stdout,
    );
}
unsafe fn main_0(mut c: std::os::raw::c_int, mut v: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut size: std::os::raw::c_int = 0;
    let mut depth: std::os::raw::c_int = 0;
    depth = if c > 1 as std::os::raw::c_int {
        atoi(*v.offset(1 as std::os::raw::c_int as isize))
    } else {
        10 as std::os::raw::c_int
    };
    size = (1 as std::os::raw::c_int) << depth;
    fprintf(
        stderr,
        b"size: %d depth: %d\n\0" as *const u8 as *const std::os::raw::c_char,
        size,
        depth,
    );
    sierp(size as std::os::raw::c_long, depth + 2 as std::os::raw::c_int);
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

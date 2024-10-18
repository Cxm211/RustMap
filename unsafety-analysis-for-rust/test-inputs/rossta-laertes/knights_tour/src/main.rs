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
    fn getchar() -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
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
pub type cell = std::os::raw::c_uchar;
#[no_mangle]
pub static mut dx: [std::os::raw::c_int; 8] = [
    -(2 as std::os::raw::c_int),
    -(2 as std::os::raw::c_int),
    -(1 as std::os::raw::c_int),
    1 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    1 as std::os::raw::c_int,
    -(1 as std::os::raw::c_int),
];
#[no_mangle]
pub static mut dy: [std::os::raw::c_int; 8] = [
    -(1 as std::os::raw::c_int),
    1 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    1 as std::os::raw::c_int,
    -(1 as std::os::raw::c_int),
    -(2 as std::os::raw::c_int),
    -(2 as std::os::raw::c_int),
];
#[no_mangle]
pub unsafe extern "C" fn init_board(
    mut w: std::os::raw::c_int,
    mut h: std::os::raw::c_int,
    mut a: *mut *mut cell,
    mut b: *mut *mut cell,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut x: std::os::raw::c_int = 0;
    let mut y: std::os::raw::c_int = 0;
    let mut p: std::os::raw::c_int = w + 4 as std::os::raw::c_int;
    let mut q: std::os::raw::c_int = h + 4 as std::os::raw::c_int;
    let ref mut fresh0 = *a.offset(0 as std::os::raw::c_int as isize);
    *fresh0 = a.offset(q as isize) as *mut cell;
    let ref mut fresh1 = *b.offset(0 as std::os::raw::c_int as isize);
    *fresh1 = (*a.offset(0 as std::os::raw::c_int as isize)).offset(2 as std::os::raw::c_int as isize);
    i = 1 as std::os::raw::c_int;
    while i < q {
        let ref mut fresh2 = *a.offset(i as isize);
        *fresh2 = (*a.offset((i - 1 as std::os::raw::c_int) as isize)).offset(p as isize);
        let ref mut fresh3 = *b.offset(i as isize);
        *fresh3 = (*a.offset(i as isize)).offset(2 as std::os::raw::c_int as isize);
        i += 1;
        i;
    }
    memset(
        *a.offset(0 as std::os::raw::c_int as isize) as *mut u8,
        255 as std::os::raw::c_int,
        (p * q) as std::os::raw::c_ulong,
    );
    i = 0 as std::os::raw::c_int;
    while i < h {
        j = 0 as std::os::raw::c_int;
        while j < w {
            k = 0 as std::os::raw::c_int;
            while k < 8 as std::os::raw::c_int {
                x = j + dx[k as usize];
                y = i + dy[k as usize];
                if *(*b.offset((i + 2 as std::os::raw::c_int) as isize)).offset(j as isize)
                    as std::os::raw::c_int == 255 as std::os::raw::c_int
                {
                    *(*b.offset((i + 2 as std::os::raw::c_int) as isize))
                        .offset(j as isize) = 0 as std::os::raw::c_int as cell;
                }
                let ref mut fresh4 = *(*b.offset((i + 2 as std::os::raw::c_int) as isize))
                    .offset(j as isize);
                *fresh4 = (*fresh4 as std::os::raw::c_int
                    + (x >= 0 as std::os::raw::c_int && x < w && y >= 0 as std::os::raw::c_int && y < h)
                        as std::os::raw::c_int) as cell;
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn walk_board(
    mut w: std::os::raw::c_int,
    mut h: std::os::raw::c_int,
    mut x: std::os::raw::c_int,
    mut y: std::os::raw::c_int,
    mut b: *mut *mut cell,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut nx: std::os::raw::c_int = 0;
    let mut ny: std::os::raw::c_int = 0;
    let mut least: std::os::raw::c_int = 0;
    let mut steps: std::os::raw::c_int = 0 as std::os::raw::c_int;
    printf(
        b"\x1B[H\x1B[J\x1B[%d;%dH\x1B[32m[]\x1B[m\0" as *const u8 as *const std::os::raw::c_char,
        y + 1 as std::os::raw::c_int,
        1 as std::os::raw::c_int + 2 as std::os::raw::c_int * x,
    );
    loop {
        *(*b.offset(y as isize)).offset(x as isize) = 255 as std::os::raw::c_int as cell;
        i = 0 as std::os::raw::c_int;
        while i < 8 as std::os::raw::c_int {
            let ref mut fresh5 = *(*b.offset((y + dy[i as usize]) as isize))
                .offset((x + dx[i as usize]) as isize);
            *fresh5 = (*fresh5).wrapping_sub(1);
            *fresh5;
            i += 1;
            i;
        }
        least = 255 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < 8 as std::os::raw::c_int {
            if (*(*b.offset((y + dy[i as usize]) as isize))
                .offset((x + dx[i as usize]) as isize) as std::os::raw::c_int) < least
            {
                nx = x + dx[i as usize];
                ny = y + dy[i as usize];
                least = *(*b.offset(ny as isize)).offset(nx as isize) as std::os::raw::c_int;
            }
            i += 1;
            i;
        }
        if least > 7 as std::os::raw::c_int {
            printf(
                b"\x1B[%dH\0" as *const u8 as *const std::os::raw::c_char,
                h + 2 as std::os::raw::c_int,
            );
            return (steps == w * h - 1 as std::os::raw::c_int) as std::os::raw::c_int;
        }
        let fresh6 = steps;
        steps = steps + 1;
        if fresh6 != 0 {
            printf(
                b"\x1B[%d;%dH[]\0" as *const u8 as *const std::os::raw::c_char,
                y + 1 as std::os::raw::c_int,
                1 as std::os::raw::c_int + 2 as std::os::raw::c_int * x,
            );
        }
        x = nx;
        y = ny;
        printf(
            b"\x1B[%d;%dH\x1B[31m[]\x1B[m\0" as *const u8 as *const std::os::raw::c_char,
            y + 1 as std::os::raw::c_int,
            1 as std::os::raw::c_int + 2 as std::os::raw::c_int * x,
        );
        fflush(stdout);
        usleep(120000 as std::os::raw::c_int as __useconds_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn solve(mut w: std::os::raw::c_int, mut h: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut x: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut y: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut a: *mut *mut cell = 0 as *mut *mut cell;
    let mut b: *mut *mut cell = core::ptr::null_mut();
    a = malloc(
        (((w + 4 as std::os::raw::c_int) * (h + 4 as std::os::raw::c_int)) as std::os::raw::c_ulong)
            .wrapping_add(
                (std::mem::size_of::<*mut cell>() as std::os::raw::c_ulong)
                    .wrapping_mul((h + 4 as std::os::raw::c_int) as std::os::raw::c_ulong),
            ),
    ) as *mut *mut cell;
    b = malloc(
        ((h + 4 as std::os::raw::c_int) as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<*mut cell>() as std::os::raw::c_ulong),
    ) as *mut *mut cell;
    loop {
        init_board(w, h, a, b);
        if walk_board(w, h, x, y, b.offset(2 as std::os::raw::c_int as isize)) != 0 {
            printf(b"Success!\n\0" as *const u8 as *const std::os::raw::c_char);
            return 1 as std::os::raw::c_int;
        }
        x += 1;
        if x >= w {
            x = 0 as std::os::raw::c_int;
            y += 1;
            y;
        }
        if y >= h {
            printf(b"Failed to find a solution\n\0" as *const u8 as *const std::os::raw::c_char);
            return 0 as std::os::raw::c_int;
        }
        printf(
            b"Any key to try next start position\0" as *const u8 as *const std::os::raw::c_char,
        );
        getchar();
    };
}
unsafe fn main_0(mut c: std::os::raw::c_int, mut v: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut w: std::os::raw::c_int = 0;
    let mut h: std::os::raw::c_int = 0;
    if c < 2 as std::os::raw::c_int
        || {
            w = atoi(*v.offset(1 as std::os::raw::c_int as isize));
            w <= 0 as std::os::raw::c_int
        }
    {
        w = 8 as std::os::raw::c_int;
    }
    if c < 3 as std::os::raw::c_int
        || {
            h = atoi(*v.offset(2 as std::os::raw::c_int as isize));
            h <= 0 as std::os::raw::c_int
        }
    {
        h = w;
    }
    solve(w, h);
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

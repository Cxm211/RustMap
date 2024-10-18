#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fwrite(
        _: *const u8,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
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
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut sandPileEdge: std::os::raw::c_int = 0;
    let mut centerPileHeight: std::os::raw::c_int = 0;
    let mut processAgain: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut top: std::os::raw::c_int = 0;
    let mut down: std::os::raw::c_int = 0;
    let mut left: std::os::raw::c_int = 0;
    let mut right: std::os::raw::c_int = 0;
    let mut sandPile: *mut *mut std::os::raw::c_int = core::ptr::null_mut();
    let mut fileName: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    static mut colour: [std::os::raw::c_uchar; 3] = [0; 3];
    if argc != 3 as std::os::raw::c_int {
        printf(
            b"Usage: %s <Sand pile side> <Center pile height>\0" as *const u8
                as *const std::os::raw::c_char,
            *argv.offset(0 as std::os::raw::c_int as isize),
        );
        return 0 as std::os::raw::c_int;
    }
    sandPileEdge = atoi(*argv.offset(1 as std::os::raw::c_int as isize));
    centerPileHeight = atoi(*argv.offset(2 as std::os::raw::c_int as isize));
    if sandPileEdge <= 0 as std::os::raw::c_int || centerPileHeight <= 0 as std::os::raw::c_int {
        printf(
            b"Sand pile and center pile dimensions must be positive integers.\0"
                as *const u8 as *const std::os::raw::c_char,
        );
        return 0 as std::os::raw::c_int;
    }
    sandPile = malloc(
        (sandPileEdge as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < sandPileEdge {
        let ref mut fresh0 = *sandPile.offset(i as isize);
        *fresh0 = calloc(
            sandPileEdge as std::os::raw::c_ulong,
            std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_int;
        i += 1;
        i;
    }
    *(*sandPile.offset((sandPileEdge / 2 as std::os::raw::c_int) as isize))
        .offset((sandPileEdge / 2 as std::os::raw::c_int) as isize) = centerPileHeight;
    printf(b"Initial sand pile :\n\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < sandPileEdge {
        j = 0 as std::os::raw::c_int;
        while j < sandPileEdge {
            printf(
                b"%3d\0" as *const u8 as *const std::os::raw::c_char,
                *(*sandPile.offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    while processAgain == 1 as std::os::raw::c_int {
        processAgain = 0 as std::os::raw::c_int;
        top = 0 as std::os::raw::c_int;
        down = 0 as std::os::raw::c_int;
        left = 0 as std::os::raw::c_int;
        right = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < sandPileEdge {
            j = 0 as std::os::raw::c_int;
            while j < sandPileEdge {
                if *(*sandPile.offset(i as isize)).offset(j as isize) >= 4 as std::os::raw::c_int
                {
                    if i - 1 as std::os::raw::c_int >= 0 as std::os::raw::c_int {
                        top = 1 as std::os::raw::c_int;
                        *(*sandPile.offset((i - 1 as std::os::raw::c_int) as isize))
                            .offset(j as isize) += 1 as std::os::raw::c_int;
                        if *(*sandPile.offset((i - 1 as std::os::raw::c_int) as isize))
                            .offset(j as isize) >= 4 as std::os::raw::c_int
                        {
                            processAgain = 1 as std::os::raw::c_int;
                        }
                    }
                    if (i + 1 as std::os::raw::c_int) < sandPileEdge {
                        down = 1 as std::os::raw::c_int;
                        *(*sandPile.offset((i + 1 as std::os::raw::c_int) as isize))
                            .offset(j as isize) += 1 as std::os::raw::c_int;
                        if *(*sandPile.offset((i + 1 as std::os::raw::c_int) as isize))
                            .offset(j as isize) >= 4 as std::os::raw::c_int
                        {
                            processAgain = 1 as std::os::raw::c_int;
                        }
                    }
                    if j - 1 as std::os::raw::c_int >= 0 as std::os::raw::c_int {
                        left = 1 as std::os::raw::c_int;
                        *(*sandPile.offset(i as isize))
                            .offset((j - 1 as std::os::raw::c_int) as isize) += 1 as std::os::raw::c_int;
                        if *(*sandPile.offset(i as isize))
                            .offset((j - 1 as std::os::raw::c_int) as isize) >= 4 as std::os::raw::c_int
                        {
                            processAgain = 1 as std::os::raw::c_int;
                        }
                    }
                    if (j + 1 as std::os::raw::c_int) < sandPileEdge {
                        right = 1 as std::os::raw::c_int;
                        *(*sandPile.offset(i as isize))
                            .offset((j + 1 as std::os::raw::c_int) as isize) += 1 as std::os::raw::c_int;
                        if *(*sandPile.offset(i as isize))
                            .offset((j + 1 as std::os::raw::c_int) as isize) >= 4 as std::os::raw::c_int
                        {
                            processAgain = 1 as std::os::raw::c_int;
                        }
                    }
                    *(*sandPile.offset(i as isize)).offset(j as isize)
                        -= top + down + left + right;
                    if *(*sandPile.offset(i as isize)).offset(j as isize)
                        >= 4 as std::os::raw::c_int
                    {
                        processAgain = 1 as std::os::raw::c_int;
                    }
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    printf(b"Final sand pile : \n\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < sandPileEdge {
        j = 0 as std::os::raw::c_int;
        while j < sandPileEdge {
            printf(
                b"%3d\0" as *const u8 as *const std::os::raw::c_char,
                *(*sandPile.offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    fileName = malloc(
        (strlen(*argv.offset(1 as std::os::raw::c_int as isize)))
            .wrapping_add(strlen(*argv.offset(2 as std::os::raw::c_int as isize)))
            .wrapping_add(23 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    strcpy(fileName, b"Final_Sand_Pile_\0" as *const u8 as *const std::os::raw::c_char);
    strcat(fileName, *argv.offset(1 as std::os::raw::c_int as isize));
    strcat(fileName, b"_\0" as *const u8 as *const std::os::raw::c_char);
    strcat(fileName, *argv.offset(2 as std::os::raw::c_int as isize));
    strcat(fileName, b".ppm\0" as *const u8 as *const std::os::raw::c_char);
    let mut fp: *mut FILE = fopen(fileName, b"wb\0" as *const u8 as *const std::os::raw::c_char);
    fprintf(
        fp,
        b"P6\n%d %d\n255\n\0" as *const u8 as *const std::os::raw::c_char,
        sandPileEdge,
        sandPileEdge,
    );
    i = 0 as std::os::raw::c_int;
    while i < sandPileEdge {
        j = 0 as std::os::raw::c_int;
        while j < sandPileEdge {
            colour[0 as std::os::raw::c_int
                as usize] = ((*(*sandPile.offset(i as isize)).offset(j as isize) + i)
                % 256 as std::os::raw::c_int) as std::os::raw::c_uchar;
            colour[1 as std::os::raw::c_int
                as usize] = ((*(*sandPile.offset(i as isize)).offset(j as isize) + j)
                % 256 as std::os::raw::c_int) as std::os::raw::c_uchar;
            colour[2 as std::os::raw::c_int
                as usize] = ((*(*sandPile.offset(i as isize)).offset(j as isize) + i * j)
                % 256 as std::os::raw::c_int) as std::os::raw::c_uchar;
            fwrite(
                colour.as_mut_ptr() as *const u8,
                1 as std::os::raw::c_int as std::os::raw::c_ulong,
                3 as std::os::raw::c_int as std::os::raw::c_ulong,
                fp,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fclose(fp);
    printf(
        b"\nImage file written to %s\n\0" as *const u8 as *const std::os::raw::c_char,
        fileName,
    );
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

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
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
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
pub unsafe extern "C" fn process(
    mut lineNum: std::os::raw::c_int,
    mut buffer: *mut std::os::raw::c_char,
) {
    let mut days: [[std::os::raw::c_char; 64]; 7] = [[0; 64]; 7];
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut d: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut j: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while *buffer.offset(i as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int {
        if *buffer.offset(i as isize) as std::os::raw::c_int == ' ' as i32 {
            days[d as usize][j as usize] = '\0' as i32 as std::os::raw::c_char;
            d += 1;
            d;
            j = 0 as std::os::raw::c_int;
        } else if *buffer.offset(i as isize) as std::os::raw::c_int == '\n' as i32
            || *buffer.offset(i as isize) as std::os::raw::c_int == '\r' as i32
        {
            days[d as usize][j as usize] = '\0' as i32 as std::os::raw::c_char;
            d += 1;
            d;
            break;
        } else {
            days[d as usize][j as usize] = *buffer.offset(i as isize);
            j += 1;
            j;
        }
        if d >= 7 as std::os::raw::c_int {
            printf(
                b"There aren't 7 days in line %d\n\0" as *const u8
                    as *const std::os::raw::c_char,
                lineNum,
            );
            return;
        }
        i += 1;
        i;
    }
    if *buffer.offset(i as isize) as std::os::raw::c_int == '\0' as i32 {
        days[d as usize][j as usize] = '\0' as i32 as std::os::raw::c_char;
        d += 1;
        d;
    }
    if d < 7 as std::os::raw::c_int {
        printf(
            b"There aren't 7 days in line %d\n\0" as *const u8 as *const std::os::raw::c_char,
            lineNum,
        );
        return;
    } else {
        let mut len: std::os::raw::c_int = 0 as std::os::raw::c_int;
        len = 1 as std::os::raw::c_int;
        while len < 64 as std::os::raw::c_int {
            let mut current_block_35: u64;
            let mut d1: std::os::raw::c_int = 0;
            d1 = 0 as std::os::raw::c_int;
            's_113: loop {
                if !(d1 < 7 as std::os::raw::c_int) {
                    current_block_35 = 18153031941552419006;
                    break;
                }
                let mut d2: std::os::raw::c_int = 0;
                d2 = d1 + 1 as std::os::raw::c_int;
                while d2 < 7 as std::os::raw::c_int {
                    let mut unique: std::os::raw::c_int = 0 as std::os::raw::c_int;
                    i = 0 as std::os::raw::c_int;
                    while i < len {
                        if days[d1 as usize][i as usize] as std::os::raw::c_int
                            != days[d2 as usize][i as usize] as std::os::raw::c_int
                        {
                            unique = 1 as std::os::raw::c_int;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    if unique == 0 {
                        current_block_35 = 10891380440665537214;
                        break 's_113;
                    }
                    d2 += 1;
                    d2;
                }
                d1 += 1;
                d1;
            }
            match current_block_35 {
                10891380440665537214 => {}
                _ => {
                    printf(b"%2d \0" as *const u8 as *const std::os::raw::c_char, len);
                    i = 0 as std::os::raw::c_int;
                    while i < 7 as std::os::raw::c_int {
                        printf(
                            b" %s\0" as *const u8 as *const std::os::raw::c_char,
                            (days[i as usize]).as_mut_ptr(),
                        );
                        i += 1;
                        i;
                    }
                    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
                    return;
                }
            }
            len += 1;
            len;
        }
    }
    printf(
        b"Failed to find uniqueness within the bounds.\0" as *const u8
            as *const std::os::raw::c_char,
    );
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut buffer: [std::os::raw::c_char; 1024] = [0; 1024];
    let mut lineNum: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut len: std::os::raw::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(
        b"days_of_week.txt\0" as *const u8 as *const std::os::raw::c_char,
        b"r\0" as *const u8 as *const std::os::raw::c_char,
    );
    loop {
        memset(
            buffer.as_mut_ptr() as *mut u8,
            0 as std::os::raw::c_int,
            std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong,
        );
        fgets(
            buffer.as_mut_ptr(),
            std::mem::size_of::<[std::os::raw::c_char; 1024]>() as std::os::raw::c_ulong
                as std::os::raw::c_int,
            fp,
        );
        len = strlen(buffer.as_mut_ptr()) as std::os::raw::c_int;
        if len == 0 as std::os::raw::c_int
            || buffer[(len - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int == '\0' as i32
        {
            break;
        }
        let fresh0 = lineNum;
        lineNum = lineNum + 1;
        process(fresh0, buffer.as_mut_ptr());
    }
    fclose(fp);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

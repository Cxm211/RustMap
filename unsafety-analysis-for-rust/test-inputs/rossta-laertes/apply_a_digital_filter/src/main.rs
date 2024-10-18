#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_double;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strtok(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
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
pub struct vector {
    pub values: * mut std::os::raw::c_float,
    pub size: std::os::raw::c_int,
}
impl std::default::Default for vector {
    fn default() -> Self {
        vector {
        values: 0 as * mut std::os::raw::c_float,
        size: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn extractVector(mut str: *mut std::os::raw::c_char) -> vector {
    let mut coeff: vector = vector {
        values: core::ptr::null_mut(),
        size: 0,
    };
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut count: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut token: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    while *str.offset(i as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int {
        let fresh0 = i;
        i = i + 1;
        if *str.offset(fresh0 as isize) as std::os::raw::c_int == ' ' as i32 {
            count += 1;
            count;
        }
    }
    coeff
        .values = malloc(
        (count as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_float;
    coeff.size = count;
    token = strtok(str, b" \0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while !token.is_null() {
        let fresh1 = i;
        i = i + 1;
        *(coeff.values).offset(fresh1 as isize) = atof(token) as std::os::raw::c_float;
        token = strtok(
            0 as *mut std::os::raw::c_char,
            b" \0" as *const u8 as *const std::os::raw::c_char,
        );
    }
    return coeff;
}
#[no_mangle]
pub unsafe extern "C" fn processSignalFile(mut fileName: *mut std::os::raw::c_char) -> vector {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_float = 0.;
    let mut str: [std::os::raw::c_char; 1000] = [0; 1000];
    let mut coeff1: vector = vector {
        values: core::ptr::null_mut(),
        size: 0,
    };
    let mut coeff2: vector = vector {
        values: core::ptr::null_mut(),
        size: 0,
    };
    let mut signal: vector = vector {
        values: core::ptr::null_mut(),
        size: 0,
    };
    let mut filteredSignal: vector = vector {
        values: core::ptr::null_mut(),
        size: 0,
    };
    let mut fp: *mut FILE = fopen(fileName, b"r\0" as *const u8 as *const std::os::raw::c_char);
    fgets(str.as_mut_ptr(), 1000 as std::os::raw::c_int, fp);
    coeff1 = extractVector(str.as_mut_ptr());
    fgets(str.as_mut_ptr(), 1000 as std::os::raw::c_int, fp);
    coeff2 = extractVector(str.as_mut_ptr());
    fgets(str.as_mut_ptr(), 1000 as std::os::raw::c_int, fp);
    signal = extractVector(str.as_mut_ptr());
    fclose(fp);
    filteredSignal
        .values = calloc(
        signal.size as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_float;
    filteredSignal.size = signal.size;
    i = 0 as std::os::raw::c_int;
    while i < signal.size {
        sum = 0 as std::os::raw::c_int as std::os::raw::c_float;
        j = 0 as std::os::raw::c_int;
        while j < coeff2.size {
            if i - j >= 0 as std::os::raw::c_int {
                sum
                    += *(coeff2.values).offset(j as isize)
                        * *(signal.values).offset((i - j) as isize);
            }
            j += 1;
            j;
        }
        j = 0 as std::os::raw::c_int;
        while j < coeff1.size {
            if i - j >= 0 as std::os::raw::c_int {
                sum
                    -= *(coeff1.values).offset(j as isize)
                        * *(filteredSignal.values).offset((i - j) as isize);
            }
            j += 1;
            j;
        }
        sum /= *(coeff1.values).offset(0 as std::os::raw::c_int as isize);
        *(filteredSignal.values).offset(i as isize) = sum;
        i += 1;
        i;
    }
    return filteredSignal;
}
#[no_mangle]
pub unsafe extern "C" fn printVector(mut v: vector, mut outputFile: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    if outputFile.is_null() {
        printf(b"[\0" as *const u8 as *const std::os::raw::c_char);
        i = 0 as std::os::raw::c_int;
        while i < v.size {
            printf(
                b"%.12f, \0" as *const u8 as *const std::os::raw::c_char,
                *(v.values).offset(i as isize) as std::os::raw::c_double,
            );
            i += 1;
            i;
        }
        printf(b"\x08\x08]\0" as *const u8 as *const std::os::raw::c_char);
    } else {
        let mut fp: *mut FILE = fopen(
            outputFile,
            b"w\0" as *const u8 as *const std::os::raw::c_char,
        );
        i = 0 as std::os::raw::c_int;
        while i < v.size - 1 as std::os::raw::c_int {
            fprintf(
                fp,
                b"%.12f, \0" as *const u8 as *const std::os::raw::c_char,
                *(v.values).offset(i as isize) as std::os::raw::c_double,
            );
            i += 1;
            i;
        }
        fprintf(
            fp,
            b"%.12f\0" as *const u8 as *const std::os::raw::c_char,
            *(v.values).offset(i as isize) as std::os::raw::c_double,
        );
        fclose(fp);
    };
}
unsafe fn main_0(
    mut argC: std::os::raw::c_int,
    mut argV: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut str: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if argC < 2 as std::os::raw::c_int || argC > 3 as std::os::raw::c_int {
        printf(
            b"Usage : %s <name of signal data file and optional output file.>\0"
                as *const u8 as *const std::os::raw::c_char,
            *argV.offset(0 as std::os::raw::c_int as isize),
        );
    } else {
        if argC != 2 as std::os::raw::c_int {
            str = malloc(
                (strlen(*argV.offset(2 as std::os::raw::c_int as isize)))
                    .wrapping_add(strlen(str))
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_mul(
                        std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong,
                    ),
            ) as *mut std::os::raw::c_char;
            strcpy(str, b"written to \0" as *const u8 as *const std::os::raw::c_char);
        }
        printf(
            b"Filtered signal %s\0" as *const u8 as *const std::os::raw::c_char,
            if argC == 2 as std::os::raw::c_int {
                b"is:\n\0" as *const u8 as *const std::os::raw::c_char
            } else {
                strcat(str, *argV.offset(2 as std::os::raw::c_int as isize))
                    as *const std::os::raw::c_char
            },
        );
        printVector(
            processSignalFile(*argV.offset(1 as std::os::raw::c_int as isize)),
            *argV.offset(2 as std::os::raw::c_int as isize),
        );
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

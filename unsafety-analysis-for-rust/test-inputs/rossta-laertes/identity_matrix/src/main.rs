#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn exit(_: std::os::raw::c_int) -> !;
    fn abort() -> !;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    if argc < 2 as std::os::raw::c_int {
        printf(
            b"usage: identitymatrix <number of rows>\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
        exit(1 as std::os::raw::c_int);
    }
    let mut rowsize: std::os::raw::c_int = atoi(*argv.offset(1 as std::os::raw::c_int as isize));
    if rowsize < 0 as std::os::raw::c_int {
        printf(
            b"Dimensions of matrix cannot be negative\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
        exit(1 as std::os::raw::c_int);
    }
    let mut numElements: std::os::raw::c_int = rowsize * rowsize;
    if numElements < rowsize {
        printf(
            b"Squaring %d caused result to overflow to %d.\n\0" as *const u8
                as *const std::os::raw::c_char,
            rowsize,
            numElements,
        );
        abort();
    }
    let mut matrix: *mut *mut std::os::raw::c_int = calloc(
        numElements as std::os::raw::c_ulong,
        std::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut *mut std::os::raw::c_int;
    if matrix.is_null() {
        printf(
            b"Failed to allocate %d elements of %d bytes each\n\0" as *const u8
                as *const std::os::raw::c_char,
            numElements,
            std::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong,
        );
        abort();
    }
    let mut row: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while row < rowsize as std::os::raw::c_uint {
        let ref mut fresh0 = *matrix.offset(row as isize);
        *fresh0 = calloc(
            numElements as std::os::raw::c_ulong,
            std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_int;
        if (*matrix.offset(row as isize)).is_null() {
            printf(
                b"Failed to allocate %d elements of %d bytes each\n\0" as *const u8
                    as *const std::os::raw::c_char,
                numElements,
                std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
            );
            abort();
        }
        *(*matrix.offset(row as isize)).offset(row as isize) = 1 as std::os::raw::c_int;
        row = row.wrapping_add(1);
        row;
    }
    printf(b"Matrix is: \n\0" as *const u8 as *const std::os::raw::c_char);
    let mut row_0: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while row_0 < rowsize as std::os::raw::c_uint {
        let mut column: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while column < rowsize as std::os::raw::c_uint {
            printf(
                b"%d \0" as *const u8 as *const std::os::raw::c_char,
                *(*matrix.offset(row_0 as isize)).offset(column as isize),
            );
            column = column.wrapping_add(1);
            column;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        row_0 = row_0.wrapping_add(1);
        row_0;
    }
    return 0;
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

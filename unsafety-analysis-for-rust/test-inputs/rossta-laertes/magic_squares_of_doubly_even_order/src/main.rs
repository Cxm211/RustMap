#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn __ctype_b_loc() -> *mut *const std::os::raw::c_ushort;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[no_mangle]
pub unsafe extern "C" fn doublyEvenMagicSquare(
    mut n: std::os::raw::c_int,
) -> *mut *mut std::os::raw::c_int {
    if n < 4 as std::os::raw::c_int || n % 4 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        return core::ptr::null_mut();
    }
    let mut bits: std::os::raw::c_int = 38505 as std::os::raw::c_int;
    let mut size: std::os::raw::c_int = n * n;
    let mut mult: std::os::raw::c_int = n / 4 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = 0;
    let mut bitPos: std::os::raw::c_int = 0;
    let mut result: *mut *mut std::os::raw::c_int = malloc(
        (n as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < n {
        let ref mut fresh0 = *result.offset(i as isize);
        *fresh0 = malloc(
            (n as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
        i += 1;
        i;
    }
    r = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while r < n {
        c = 0 as std::os::raw::c_int;
        while c < n {
            bitPos = c / mult + r / mult * 4 as std::os::raw::c_int;
            *(*result.offset(r as isize))
                .offset(
                    c as isize,
                ) = if bits & (1 as std::os::raw::c_int) << bitPos != 0 as std::os::raw::c_int {
                i + 1 as std::os::raw::c_int
            } else {
                size - i
            };
            c += 1;
            c;
            i += 1;
            i;
        }
        r += 1;
        r;
    }
    return result;
}
#[no_mangle]
pub extern "C" fn numDigits(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut count: std::os::raw::c_int = 1 as std::os::raw::c_int;
    while n >= 10 as std::os::raw::c_int {
        n /= 10 as std::os::raw::c_int;
        count += 1;
        count;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn printMagicSquare(
    mut square: *mut *mut std::os::raw::c_int,
    mut rows: std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut baseWidth: std::os::raw::c_int = numDigits(rows * rows) + 3 as std::os::raw::c_int;
    printf(
        b"Doubly Magic Square of Order : %d and Magic Constant : %d\n\n\0" as *const u8
            as *const std::os::raw::c_char,
        rows,
        (rows * rows + 1 as std::os::raw::c_int) * rows / 2 as std::os::raw::c_int,
    );
    i = 0 as std::os::raw::c_int;
    while i < rows {
        j = 0 as std::os::raw::c_int;
        while j < rows {
            printf(
                b"%*s%d\0" as *const u8 as *const std::os::raw::c_char,
                baseWidth - numDigits(*(*square.offset(i as isize)).offset(j as isize)),
                b"\0" as *const u8 as *const std::os::raw::c_char,
                *(*square.offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
}
unsafe fn main_0(
    mut argC: std::os::raw::c_int,
    mut argV: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0;
    if argC != 2 as std::os::raw::c_int
        || *(*__ctype_b_loc())
            .offset(
                *(*argV.offset(1 as std::os::raw::c_int as isize))
                    .offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int as isize,
            ) as std::os::raw::c_int & _ISdigit as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int
            == 0 as std::os::raw::c_int
    {
        printf(
            b"Usage : %s <integer specifying rows in magic square>\0" as *const u8
                as *const std::os::raw::c_char,
            *argV.offset(0 as std::os::raw::c_int as isize),
        );
    } else {
        n = atoi(*argV.offset(1 as std::os::raw::c_int as isize));
        printMagicSquare(doublyEvenMagicSquare(n), n);
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

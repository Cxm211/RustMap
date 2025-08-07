#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
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
pub unsafe extern "C" fn oddMagicSquare(mut n: std::os::raw::c_int) -> *mut *mut std::os::raw::c_int {
    if n < 3 as std::os::raw::c_int || n % 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 0 as *mut *mut std::os::raw::c_int;
    }
    let mut value: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut squareSize: std::os::raw::c_int = n * n;
    let mut c: std::os::raw::c_int = n / 2 as std::os::raw::c_int;
    let mut r: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut result: *mut *mut std::os::raw::c_int = malloc(
        (n as std::os::raw::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < n {
        let ref mut fresh0 = *result.offset(i as isize);
        *fresh0 = malloc(
            (n as std::os::raw::c_ulong)
                .wrapping_mul(::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
        i += 1;
        i;
    }
    loop {
        value += 1;
        if !(value <= squareSize) {
            break;
        }
        *(*result.offset(r as isize)).offset(c as isize) = value;
        if r == 0 as std::os::raw::c_int {
            if c == n - 1 as std::os::raw::c_int {
                r += 1;
                r;
            } else {
                r = n - 1 as std::os::raw::c_int;
                c += 1;
                c;
            }
        } else if c == n - 1 as std::os::raw::c_int {
            r -= 1;
            r;
            c = 0 as std::os::raw::c_int;
        } else if *(*result.offset((r - 1 as std::os::raw::c_int) as isize))
            .offset((c + 1 as std::os::raw::c_int) as isize) == 0 as std::os::raw::c_int
        {
            r -= 1;
            r;
            c += 1;
            c;
        } else {
            r += 1;
            r;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn singlyEvenMagicSquare(
    mut n: std::os::raw::c_int,
) -> *mut *mut std::os::raw::c_int {
    if n < 6 as std::os::raw::c_int
        || (n - 2 as std::os::raw::c_int) % 4 as std::os::raw::c_int != 0 as std::os::raw::c_int
    {
        return 0 as *mut *mut std::os::raw::c_int;
    }
    let mut size: std::os::raw::c_int = n * n;
    let mut halfN: std::os::raw::c_int = n / 2 as std::os::raw::c_int;
    let mut subGridSize: std::os::raw::c_int = size / 4 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut subGrid: *mut *mut std::os::raw::c_int = oddMagicSquare(halfN);
    let mut gridFactors: [std::os::raw::c_int; 4] = [
        0 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    ];
    let mut result: *mut *mut std::os::raw::c_int = malloc(
        (n as std::os::raw::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < n {
        let ref mut fresh1 = *result.offset(i as isize);
        *fresh1 = malloc(
            (n as std::os::raw::c_ulong)
                .wrapping_mul(::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
        i += 1;
        i;
    }
    let mut r: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while r < n {
        let mut c: std::os::raw::c_int = 0 as std::os::raw::c_int;
        while c < n {
            let mut grid: std::os::raw::c_int = r / halfN * 2 as std::os::raw::c_int + c / halfN;
            *(*result.offset(r as isize))
                .offset(
                    c as isize,
                ) = *(*subGrid.offset((r % halfN) as isize))
                .offset((c % halfN) as isize);
            *(*result.offset(r as isize)).offset(c as isize)
                += gridFactors[grid as usize] * subGridSize;
            c += 1;
            c;
        }
        r += 1;
        r;
    }
    let mut nColsLeft: std::os::raw::c_int = halfN / 2 as std::os::raw::c_int;
    let mut nColsRight: std::os::raw::c_int = nColsLeft - 1 as std::os::raw::c_int;
    let mut r_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while r_0 < halfN {
        let mut c_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
        while c_0 < n {
            if c_0 < nColsLeft || c_0 >= n - nColsRight
                || c_0 == nColsLeft && r_0 == nColsLeft
            {
                if !(c_0 == 0 as std::os::raw::c_int && r_0 == nColsLeft) {
                    let mut tmp: std::os::raw::c_int = *(*result.offset(r_0 as isize))
                        .offset(c_0 as isize);
                    *(*result.offset(r_0 as isize))
                        .offset(
                            c_0 as isize,
                        ) = *(*result.offset((r_0 + halfN) as isize))
                        .offset(c_0 as isize);
                    *(*result.offset((r_0 + halfN) as isize)).offset(c_0 as isize) = tmp;
                }
            }
            c_0 += 1;
            c_0;
        }
        r_0 += 1;
        r_0;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn numDigits(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
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
    i = 0 as std::os::raw::c_int;
    while i < rows {
        j = 0 as std::os::raw::c_int;
        while j < rows {
            printf(
                b"%*s%d\0" as *const u8 as *const std::os::raw::c_char,
                rows - numDigits(*(*square.offset(i as isize)).offset(j as isize)),
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
    printf(
        b"\nMagic constant: %d \0" as *const u8 as *const std::os::raw::c_char,
        (rows * rows + 1 as std::os::raw::c_int) * rows / 2 as std::os::raw::c_int,
    );
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
        printMagicSquare(singlyEvenMagicSquare(n), n);
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
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

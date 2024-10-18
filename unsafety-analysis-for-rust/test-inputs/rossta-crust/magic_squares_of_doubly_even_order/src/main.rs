#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

//use c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const u16;
}
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
#[no_mangle]
pub extern "C" fn doublyEvenMagicSquare(mut n: i32) -> *mut *mut i32 {
    unsafe {
        if n < 4 || n % 4 != 0 {
            return 0 as *mut *mut i32;
        }
        let mut bits: i32 = 38505;
        let mut size: i32 = n * n;
        let mut mult: i32 = n / 4;
        let mut i: i32 = 0;
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut bitPos: i32 = 0;
        let mut result: *mut *mut i32 =
            malloc((n as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64))
                as *mut *mut i32;
        i = 0;
        while i < n {
            let ref mut fresh0 = *result.offset(i as isize);
            *fresh0 =
                malloc((n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)) as *mut i32;
            i += 1;
            i;
        }
        r = 0;
        i = 0;
        while r < n {
            c = 0;
            while c < n {
                bitPos = c / mult + r / mult * 4;
                *(*result.offset(r as isize)).offset(c as isize) = if bits & 1 << bitPos != 0 {
                    i + 1
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
}

#[no_mangle]
pub extern "C" fn numDigits(mut n: i32) -> i32 {
    let mut count: i32 = 1;
    while n >= 10 {
        n /= 10;
        count += 1;
        count;
    }
    return count;
}

#[no_mangle]
pub extern "C" fn printMagicSquare(mut square: *mut *mut i32, mut rows: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut baseWidth: i32 = numDigits(rows * rows) + 3;
        print!(
            "Doubly Magic Square of Order : {} and Magic Constant : {}\n\n",
            rows,
            (rows * rows + 1) * rows / 2
        );
        i = 0;
        while i < rows {
            j = 0;
            while j < rows {
                print!(
                    "{1:0$}{2:}",
                    (baseWidth - numDigits(*(*square.offset(i as isize)).offset(j as isize))).abs()
                        as usize,
                    "\0",
                    *(*square.offset(i as isize)).offset(j as isize)
                );
                j += 1;
                j;
            }
            print!("\n");
            i += 1;
            i;
        }
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
    unsafe {
        let mut n: i32 = 0;
        if argC != 2
            || *(*__ctype_b_loc())
                .offset(*(*argV.offset(1 as isize)).offset(0 as isize) as i32 as isize)
                as i32
                & _ISdigit as i32
                == 0
        {
            print!(
                "Usage : {} <integer specifying rows in magic square>",
                build_str_from_raw_ptr(*argV.offset(0 as isize) as *mut u8)
            );
        } else {
            n = atoi(*argV.offset(1 as isize));
            printMagicSquare(doublyEvenMagicSquare(n), n);
        }
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}

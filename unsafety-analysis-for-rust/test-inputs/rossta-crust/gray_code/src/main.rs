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
extern "C" {}
#[no_mangle]
pub extern "C" fn gray_encode(mut n: i32) -> i32 {
    return n ^ n >> 1;
}

#[no_mangle]
pub extern "C" fn gray_decode(mut n: i32) -> i32 {
    let mut p: i32 = n;
    loop {
        n >>= 1;
        if !(n != 0) {
            break;
        }
        p ^= n;
    }
    return p;
}

#[no_mangle]
pub extern "C" fn fmtbool(mut n: i32, mut buf: *mut i8) {
    unsafe {
        let mut b: *mut i8 = buf.offset(5 as isize);
        *b = 0;
        loop {
            b = b.offset(-1);
            *b = ('0' as i32 + (n & 1i32)) as i8;
            n >>= 1;
            if !(b != buf) {
                break;
            }
        }
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let mut bi: [i8; 6] = [0; 6];
        let mut bg: [i8; 6] = [0; 6];
        let mut bb: [i8; 6] = [0; 6];
        i = 0;
        while i < 32 {
            g = gray_encode(i);
            b = gray_decode(g);
            fmtbool(i, bi.as_mut_ptr());
            fmtbool(g, bg.as_mut_ptr());
            fmtbool(b, bb.as_mut_ptr());
            print!(
                "{:2} : {:5} => {:5} => {:5} : {:2}\n",
                i,
                build_str_from_raw_ptr(bi.as_mut_ptr() as *mut u8),
                build_str_from_raw_ptr(bg.as_mut_ptr() as *mut u8),
                build_str_from_raw_ptr(bb.as_mut_ptr() as *mut u8),
                b
            );
            i += 1;
            i;
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

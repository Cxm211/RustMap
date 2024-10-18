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
//use c2rust_out::*;
extern "C" {
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut a2: i32 = 0;
        let mut s: i32 = 3;
        let mut s1: i32 = 0;
        let mut s2: i32 = 0;
        let mut r: [i32; 2201] = [0; 2201];
        memset(
            r.as_mut_ptr() as *mut libc::c_void,
            0,
            ::core::mem::size_of::<[i32; 2201]>() as u64,
        );
        let mut ab: *mut i32 = calloc(
            (2200 * 2200 * 2 + 1i32) as u64,
            ::core::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        a = 1;
        while a <= 2200 {
            a2 = a * a;
            b = a;
            while b <= 2200 {
                *ab.offset((a2 + b * b) as isize) = 1;
                b += 1;
                b;
            }
            a += 1;
            a;
        }
        c = 1;
        while c <= 2200 {
            s1 = s;
            s += 2;
            s2 = s;
            d = c + 1;
            while d <= 2200 {
                if *ab.offset(s1 as isize) != 0 {
                    r[d as usize] = 1;
                }
                s1 += s2;
                s2 += 2;
                d += 1;
                d;
            }
            c += 1;
            c;
        }
        d = 1;
        while d <= 2200 {
            if r[d as usize] == 0 {
                print!("{} ", d);
            }
            d += 1;
            d;
        }
        print!("\n");
        free(ab as *mut libc::c_void);
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

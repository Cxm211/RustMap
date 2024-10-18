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
extern "C" {}
#[no_mangle]
pub extern "C" fn is_prime(mut n: u32) -> i32 {
    if n.wrapping_rem(2) == 0 || n.wrapping_rem(3) == 0 {
        return 0;
    }
    let mut p: u32 = 1;
    while p.wrapping_mul(p) < n {
        p = (p).wrapping_add(4) as u32;
        if n.wrapping_rem(p) == 0 || {
            p = (p).wrapping_add(2) as u32;
            n.wrapping_rem(p) == 0
        } {
            return 0;
        }
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn reverse(mut n: u32) -> u32 {
    let mut r: u32 = 0;
    r = 0;
    while n != 0 {
        r = r.wrapping_mul(10).wrapping_add(n.wrapping_rem(10));
        n = (n as u32).wrapping_div(10) as u32;
    }
    return r;
}

#[no_mangle]
pub extern "C" fn is_emirp(mut n: u32) -> i32 {
    let mut r: u32 = reverse(n);
    return (r != n && is_prime(n) != 0 && is_prime(r) != 0) as i32;
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut x: u32 = 0;
        let mut c: u32 = 0;
        match argc {
            1 => {
                x = 11;
                while c < 20 {
                    if is_emirp(x) != 0 {
                        print!(" {}", x);
                        c = c.wrapping_add(1);
                        c;
                    }
                    x = (x).wrapping_add(2) as u32;
                }
            }
            2 => {
                x = 7701;
                while x < 8000 {
                    if is_emirp(x) != 0 {
                        print!(" {}", x);
                    }
                    x = (x).wrapping_add(2) as u32;
                }
            }
            _ => {
                x = 11;
                loop {
                    if is_emirp(x) != 0 && {
                        c = c.wrapping_add(1);
                        c == 10000
                    } {
                        print!("{}", x);
                        break;
                    } else {
                        x = (x).wrapping_add(2) as u32;
                    }
                }
            }
        }
        print!("{}", '\n' as i32);
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

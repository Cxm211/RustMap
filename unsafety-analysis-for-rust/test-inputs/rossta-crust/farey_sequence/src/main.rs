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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac {
    pub d: i32,
    pub n: i32,
}
#[no_mangle]
pub extern "C" fn farey(mut n: i32) {
    let mut f1: frac = {
        let mut init = frac { d: 0, n: 1 };
        init
    };
    let mut f2: frac = {
        let mut init = frac { d: 1, n: n };
        init
    };
    let mut t: frac = frac { d: 0, n: 0 };
    let mut k: i32 = 0;
    print!("{}/{} {}/{}", 0, 1, 1, n);
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2;
        f2 = {
            let mut init = frac {
                d: f2.d * k - t.d,
                n: f2.n * k - t.n,
            };
            init
        };
        print!(" {}/{}", f2.d, f2.n);
    }
    print!("{}", '\n' as i32);
}

#[no_mangle]
pub static mut cache: *mut u64 = 0 as *const u64 as *mut u64;
#[no_mangle]
pub static mut ccap: u64 = 0;
#[no_mangle]
pub extern "C" fn farey_len(mut n: i32) -> u64 {
    unsafe {
        if n as u64 >= ccap {
            let mut old: u64 = ccap;
            if ccap == 0 {
                ccap = 16;
            }
            while ccap <= n as u64 {
                ccap = (ccap as u64).wrapping_mul(2) as u64;
            }
            cache = realloc(
                cache as *mut libc::c_void,
                (::core::mem::size_of::<u64>() as u64).wrapping_mul(ccap),
            ) as *mut u64;
            memset(
                cache.offset(old as isize) as *mut libc::c_void,
                0,
                (::core::mem::size_of::<u64>() as u64).wrapping_mul(ccap.wrapping_sub(old)),
            );
        } else if *cache.offset(n as isize) != 0 {
            return *cache.offset(n as isize);
        }
    }
    let mut len: u64 = (n as u64).wrapping_mul((n + 3i32) as u64).wrapping_div(2);
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    p = 2;
    unsafe {
        while p <= n {
            q = n / (n / p) + 1;
            len = (len).wrapping_sub((farey_len(n / p)).wrapping_mul((q - p) as u64)) as u64;
            p = q;
        }
        *cache.offset(n as isize) = len;
    }
    return len;
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    n = 1;
    while n <= 11 {
        print!("{}: ", n);
        farey(n);
        n += 1;
        n;
    }
    n = 100;
    while n <= 1000 {
        print!("{}: {} items\n", n, farey_len(n));
        n += 100;
    }
    n = 10000000;
    print!("\n{}: {} items\n", n, farey_len(n));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

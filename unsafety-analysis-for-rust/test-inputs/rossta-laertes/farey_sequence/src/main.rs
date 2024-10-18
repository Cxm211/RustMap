#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac {
    pub d: std::os::raw::c_int,
    pub n: std::os::raw::c_int,
}
impl std::default::Default for frac {
    fn default() -> Self {
        frac {
        d: std::os::raw::c_int::default(),
        n: std::os::raw::c_int::default()
        }
    }
}

pub type ull = std::os::raw::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn farey(mut n: std::os::raw::c_int) {
    let mut f1: frac = {
        let mut init = frac {
            d: 0 as std::os::raw::c_int,
            n: 1 as std::os::raw::c_int,
        };
        init
    };
    let mut f2: frac = {
        let mut init = frac { d: 1 as std::os::raw::c_int, n: n };
        init
    };
    let mut t: frac = frac { d: 0, n: 0 };
    let mut k: std::os::raw::c_int = 0;
    printf(
        b"%d/%d %d/%d\0" as *const u8 as *const std::os::raw::c_char,
        0 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        n,
    );
    while f2.n > 1 as std::os::raw::c_int {
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
        printf(b" %d/%d\0" as *const u8 as *const std::os::raw::c_char, f2.d, f2.n);
    }
    putchar('\n' as i32);
}
#[no_mangle]
pub static mut cache: *mut ull = 0 as *const ull as *mut ull;
#[no_mangle]
pub static mut ccap: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn farey_len(mut n: std::os::raw::c_int) -> ull {
    if n as std::os::raw::c_ulong >= ccap {
        let mut old: size_t = ccap;
        if ccap == 0 {
            ccap = 16 as std::os::raw::c_int as size_t;
        }
        while ccap <= n as std::os::raw::c_ulong {
            ccap = (ccap as std::os::raw::c_ulong)
                .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        }
        cache = realloc(
            cache as *mut u8,
            (std::mem::size_of::<ull>() as std::os::raw::c_ulong).wrapping_mul(ccap),
        ) as *mut ull;
        memset(
            cache.offset(old as isize) as *mut u8,
            0 as std::os::raw::c_int,
            (std::mem::size_of::<ull>() as std::os::raw::c_ulong)
                .wrapping_mul(ccap.wrapping_sub(old)),
        );
    } else if *cache.offset(n as isize) != 0 {
        return *cache.offset(n as isize)
    }
    let mut len: ull = (n as ull)
        .wrapping_mul((n + 3 as std::os::raw::c_int) as std::os::raw::c_ulonglong)
        .wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulonglong);
    let mut p: std::os::raw::c_int = 0;
    let mut q: std::os::raw::c_int = 0 as std::os::raw::c_int;
    p = 2 as std::os::raw::c_int;
    while p <= n {
        q = n / (n / p) + 1 as std::os::raw::c_int;
        len = (len as std::os::raw::c_ulonglong)
            .wrapping_sub((farey_len(n / p)).wrapping_mul((q - p) as std::os::raw::c_ulonglong))
            as ull as ull;
        p = q;
    }
    *cache.offset(n as isize) = len;
    return len;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0;
    n = 1 as std::os::raw::c_int;
    while n <= 11 as std::os::raw::c_int {
        printf(b"%d: \0" as *const u8 as *const std::os::raw::c_char, n);
        farey(n);
        n += 1;
        n;
    }
    n = 100 as std::os::raw::c_int;
    while n <= 1000 as std::os::raw::c_int {
        printf(
            b"%d: %llu items\n\0" as *const u8 as *const std::os::raw::c_char,
            n,
            farey_len(n),
        );
        n += 100 as std::os::raw::c_int;
    }
    n = 10000000 as std::os::raw::c_int;
    printf(b"\n%d: %llu items\n\0" as *const u8 as *const std::os::raw::c_char, n, farey_len(n));
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

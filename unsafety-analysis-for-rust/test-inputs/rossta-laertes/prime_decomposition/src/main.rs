#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fflush(__stream: *mut FILE) -> std::os::raw::c_int;
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn fread(
        _: *mut u8,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    fn fwrite(
        _: *const u8,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: *mut FILE,
    ) -> std::os::raw::c_ulong;
    fn perror(__s: *const std::os::raw::c_char);
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn exit(_: std::os::raw::c_int) -> !;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint32_t = std::os::raw::c_uint;
pub type __uint64_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type uint8_t = std::os::raw::c_uchar;
pub type uint32_t = std::os::raw::c_uint;
pub type uint64_t = std::os::raw::c_ulong;
pub type size_t = std::os::raw::c_ulong;
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
pub type off_t = std::os::raw::c_long;
pub type pint = std::os::raw::c_uint;
pub type xint = std::os::raw::c_ulong;
#[no_mangle]
pub static mut pbits: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut bit_pos: [uint8_t; 30] = [
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 0 as std::os::raw::c_int) as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int) as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 3 as std::os::raw::c_int) as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 4 as std::os::raw::c_int) as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 5 as std::os::raw::c_int) as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 6 as std::os::raw::c_int) as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    0 as std::os::raw::c_int as uint8_t,
    ((1 as std::os::raw::c_int) << 7 as std::os::raw::c_int) as uint8_t,
];
#[no_mangle]
pub static mut rem_num: [uint8_t; 8] = [
    1 as std::os::raw::c_int as uint8_t,
    7 as std::os::raw::c_int as uint8_t,
    11 as std::os::raw::c_int as uint8_t,
    13 as std::os::raw::c_int as uint8_t,
    17 as std::os::raw::c_int as uint8_t,
    19 as std::os::raw::c_int as uint8_t,
    23 as std::os::raw::c_int as uint8_t,
    29 as std::os::raw::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn init_primes() {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: pint = 0;
    let mut tgt: pint = 4 as std::os::raw::c_int as pint;
    pbits = malloc(
        (!(0 as std::os::raw::c_int as pint))
            .wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong,
    ) as *mut uint8_t;
    if pbits.is_null() {
        perror(b"malloc\0" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    fp = fopen(
        b"primebits\0" as *const u8 as *const std::os::raw::c_char,
        b"r\0" as *const u8 as *const std::os::raw::c_char,
    );
    if !fp.is_null() {
        fread(
            pbits as *mut u8,
            1 as std::os::raw::c_int as std::os::raw::c_ulong,
            (!(0 as std::os::raw::c_int as pint))
                .wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong,
            fp,
        );
        fclose(fp);
        return;
    }
    memset(
        pbits as *mut u8,
        255 as std::os::raw::c_int,
        (!(0 as std::os::raw::c_int as pint))
            .wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong,
    );
    s = 7 as std::os::raw::c_int as pint;
    while s <= 65535 as std::os::raw::c_uint {
        if s > tgt {
            tgt = (tgt as std::os::raw::c_uint).wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_uint)
                as pint as pint;
            fprintf(stderr, b"sieve %u\n\0" as *const u8 as *const std::os::raw::c_char, s);
        }
        sieve(s);
        s = next_prime(s);
    }
    fp = fopen(
        b"primebits\0" as *const u8 as *const std::os::raw::c_char,
        b"w\0" as *const u8 as *const std::os::raw::c_char,
    );
    fwrite(
        pbits as *const u8,
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        (!(0 as std::os::raw::c_int as pint))
            .wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong,
        fp,
    );
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn is_prime(mut x: xint) -> std::os::raw::c_int {
    let mut p: pint = 0;
    if x > 5 as std::os::raw::c_int as std::os::raw::c_ulong {
        if x < !(0 as std::os::raw::c_int as pint) as std::os::raw::c_ulong {
            return *pbits
                .offset(x.wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_int
                & bit_pos[x.wrapping_rem(30 as std::os::raw::c_int as std::os::raw::c_ulong) as usize]
                    as std::os::raw::c_int;
        }
        p = 2 as std::os::raw::c_int as pint;
        while p != 0 && (p as xint).wrapping_mul(p as std::os::raw::c_ulong) <= x {
            if x.wrapping_rem(p as std::os::raw::c_ulong) == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                return 0 as std::os::raw::c_int;
            }
            p = next_prime(p);
        }
        return 1 as std::os::raw::c_int;
    }
    return (x == 2 as std::os::raw::c_int as std::os::raw::c_ulong
        || x == 3 as std::os::raw::c_int as std::os::raw::c_ulong
        || x == 5 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sieve(mut p: pint) {
    let mut b: [std::os::raw::c_uchar; 8] = [0; 8];
    let mut ofs: [off_t; 8] = [0; 8];
    let mut i: std::os::raw::c_int = 0;
    let mut q: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        q = (rem_num[i as usize] as std::os::raw::c_uint).wrapping_mul(p) as std::os::raw::c_int;
        b[i
            as usize] = !(bit_pos[(q % 30 as std::os::raw::c_int) as usize] as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        ofs[i as usize] = (q / 30 as std::os::raw::c_int) as off_t;
        i += 1;
        i;
    }
    q = ofs[1 as std::os::raw::c_int as usize] as std::os::raw::c_int;
    i = 7 as std::os::raw::c_int;
    while i != 0 {
        ofs[i as usize] -= ofs[(i - 1 as std::os::raw::c_int) as usize];
        i -= 1;
        i;
    }
    ofs[0 as std::os::raw::c_int as usize] = p as off_t;
    i = 1 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        ofs[0 as std::os::raw::c_int as usize] -= ofs[i as usize];
        i += 1;
        i;
    }
    i = 1 as std::os::raw::c_int;
    while (q as std::os::raw::c_uint)
        < (!(0 as std::os::raw::c_int as pint))
            .wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
    {
        let ref mut fresh0 = *pbits.offset(q as isize);
        *fresh0 = (*fresh0 as std::os::raw::c_int & b[i as usize] as std::os::raw::c_int) as uint8_t;
        i = i + 1 as std::os::raw::c_int & 7 as std::os::raw::c_int;
        q = (q as std::os::raw::c_long + ofs[i as usize]) as std::os::raw::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn next_prime(mut p: pint) -> pint {
    let mut addr: off_t = 0;
    let mut bits: uint8_t = 0;
    let mut rem: uint8_t = 0;
    if p > 5 as std::os::raw::c_int as std::os::raw::c_uint {
        addr = p.wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint) as off_t;
        bits = ((bit_pos[p.wrapping_rem(30 as std::os::raw::c_int as std::os::raw::c_uint) as usize]
            as std::os::raw::c_int) << 1 as std::os::raw::c_int) as uint8_t;
        rem = 0 as std::os::raw::c_int as uint8_t;
        while ((1 as std::os::raw::c_int) << rem as std::os::raw::c_int) < bits as std::os::raw::c_int {
            rem = rem.wrapping_add(1);
            rem;
        }
        while (*pbits.offset(addr as isize) as std::os::raw::c_int) < bits as std::os::raw::c_int
            || bits == 0
        {
            addr += 1;
            if addr
                >= (!(0 as std::os::raw::c_int as pint))
                    .wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_long
            {
                return 0 as std::os::raw::c_int as pint;
            }
            bits = 1 as std::os::raw::c_int as uint8_t;
            rem = 0 as std::os::raw::c_int as uint8_t;
        }
        if addr
            >= (!(0 as std::os::raw::c_int as pint))
                .wrapping_div(30 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_long
        {
            return 0 as std::os::raw::c_int as pint;
        }
        while *pbits.offset(addr as isize) as std::os::raw::c_int & bits as std::os::raw::c_int == 0 {
            rem = rem.wrapping_add(1);
            rem;
            bits = ((bits as std::os::raw::c_int) << 1 as std::os::raw::c_int) as uint8_t;
        }
        p = (addr * 30 as std::os::raw::c_int as std::os::raw::c_long
            + rem_num[rem as usize] as std::os::raw::c_long) as pint;
        return p;
    }
    match p {
        2 => return 3 as std::os::raw::c_int as pint,
        3 => return 5 as std::os::raw::c_int as pint,
        5 => return 7 as std::os::raw::c_int as pint,
        _ => {}
    }
    return 2 as std::os::raw::c_int as pint;
}
#[no_mangle]
pub unsafe extern "C" fn decompose(mut n: xint, mut f: *mut xint) -> std::os::raw::c_int {
    let mut p: pint = 0 as std::os::raw::c_int as pint;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if n <= !(0 as std::os::raw::c_int as pint) as std::os::raw::c_ulong && is_prime(n) != 0 {
        *f.offset(0 as std::os::raw::c_int as isize) = n;
        return 1 as std::os::raw::c_int;
    }
    while n >= (p as xint).wrapping_mul(p as std::os::raw::c_ulong) {
        p = next_prime(p);
        if p == 0 {
            break;
        }
        while n.wrapping_rem(p as std::os::raw::c_ulong) == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            n = (n as std::os::raw::c_ulong).wrapping_div(p as std::os::raw::c_ulong) as xint as xint;
            let fresh1 = i;
            i = i + 1;
            *f.offset(fresh1 as isize) = p as xint;
        }
    }
    if n > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        let fresh2 = i;
        i = i + 1;
        *f.offset(fresh2 as isize) = n;
    }
    return i;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut p: pint = 0 as std::os::raw::c_int as pint;
    let mut f: [xint; 63] = [0; 63];
    let mut po: xint = 0;
    init_primes();
    p = 1 as std::os::raw::c_int as pint;
    while p < 64 as std::os::raw::c_int as std::os::raw::c_uint {
        po = ((1 as std::os::raw::c_ulonglong) << p)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong) as xint;
        printf(b"2^%u - 1 = %lu\0" as *const u8 as *const std::os::raw::c_char, p, po);
        fflush(stdout);
        len = decompose(po, f.as_mut_ptr());
        if len > 1 as std::os::raw::c_int {
            i = 0 as std::os::raw::c_int;
            while i < len {
                printf(
                    b" %c %lu\0" as *const u8 as *const std::os::raw::c_char,
                    if i != 0 { 'x' as i32 } else { '=' as i32 },
                    f[i as usize],
                );
                i += 1;
                i;
            }
        }
        putchar('\n' as i32);
        p = p.wrapping_add(1);
        p;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

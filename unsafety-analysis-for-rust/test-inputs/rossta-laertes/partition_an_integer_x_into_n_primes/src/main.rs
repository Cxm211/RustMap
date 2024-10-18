#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type uint32_t = std::os::raw::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_array_tag {
    pub size: std::os::raw::c_uint,
    pub array: * mut std::os::raw::c_uint,
}
impl std::default::Default for bit_array_tag {
    fn default() -> Self {
        bit_array_tag {
        size: std::os::raw::c_uint::default(),
        array: 0 as * mut std::os::raw::c_uint
        }
    }
}

pub type bit_array = crate::bit_array_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sieve_tag {
    pub limit: std::os::raw::c_uint,
    pub not_prime: crate::bit_array_tag,
}
impl std::default::Default for sieve_tag {
    fn default() -> Self {
        sieve_tag {
        limit: std::os::raw::c_uint::default(),
        not_prime: crate::bit_array_tag::default()
        }
    }
}

pub type sieve = crate::sieve_tag;
#[no_mangle]
pub unsafe extern "C" fn bit_array_create<'a1>(
    mut b: Option<&'a1 mut crate::bit_array_tag>,
    mut size: std::os::raw::c_uint,
) -> bool {
    let mut array: *mut uint32_t = calloc(
        size
            .wrapping_add(31 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_div(32 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong,
        std::mem::size_of::<uint32_t>() as std::os::raw::c_ulong,
    ) as *mut uint32_t;
    if array.is_null() {
        return 0 as std::os::raw::c_int != 0;
    }
    (*borrow_mut(&mut b).unwrap()).size = size;
    (*borrow_mut(&mut b).unwrap()).array = array;
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn bit_array_destroy<'a1>(mut b: Option<&'a1 mut crate::bit_array_tag>) {
    free((*borrow_mut(&mut b).unwrap()).array as *mut u8);
    (*borrow_mut(&mut b).unwrap()).array = core::ptr::null_mut();
}
#[no_mangle]
pub unsafe extern "C" fn bit_array_set<'a1>(
    mut b: Option<&'a1 mut crate::bit_array_tag>,
    mut index: std::os::raw::c_uint,
    mut value: bool,
) {
    if index < (*borrow(& b).unwrap()).size {} else {
        __assert_fail(
            b"index < b->size\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            27 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 49],
                &[std::os::raw::c_char; 49],
            >(b"void bit_array_set(bit_array *, uint32_t, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_1632: {
        if index < (*borrow(& b).unwrap()).size {} else {
            __assert_fail(
                b"index < b->size\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                27 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 49],
                    &[std::os::raw::c_char; 49],
                >(b"void bit_array_set(bit_array *, uint32_t, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut p: _ = Some(&mut *((*borrow_mut(&mut b).unwrap()).array)
        .offset((index >> 5 as std::os::raw::c_int) as isize));
    let mut bit: uint32_t = ((1 as std::os::raw::c_int)
        << (index & 31 as std::os::raw::c_int as std::os::raw::c_uint)) as uint32_t;
    if value {
        *borrow_mut(&mut p).unwrap() |= bit;
    } else {
        *borrow_mut(&mut p).unwrap() &= !bit;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bit_array_get<'a1>(
    mut b: Option<&'a1 crate::bit_array_tag>,
    mut index: std::os::raw::c_uint,
) -> bool {
    if index < (*(b).clone().unwrap()).size {} else {
        __assert_fail(
            b"index < b->size\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            37 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 49],
                &[std::os::raw::c_char; 49],
            >(b"_Bool bit_array_get(const bit_array *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_1716: {
        if index < (*(b).clone().unwrap()).size {} else {
            __assert_fail(
                b"index < b->size\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                37 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 49],
                    &[std::os::raw::c_char; 49],
                >(b"_Bool bit_array_get(const bit_array *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut bit: uint32_t = ((1 as std::os::raw::c_int)
        << (index & 31 as std::os::raw::c_int as std::os::raw::c_uint)) as uint32_t;
    return *((*b.unwrap()).array).offset((index >> 5 as std::os::raw::c_int) as isize) & bit
        != 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn sieve_create<'a1>(mut s: Option<&'a1 mut crate::sieve_tag>, mut limit: std::os::raw::c_uint) -> bool {
    if !bit_array_create(
        Some(&mut (*borrow_mut(&mut s).unwrap()).not_prime),
        limit.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint),
    ) {
        return 0 as std::os::raw::c_int != 0;
    }
    bit_array_set(
        Some(&mut (*borrow_mut(&mut s).unwrap()).not_prime),
        0 as std::os::raw::c_int as uint32_t,
        1 as std::os::raw::c_int != 0,
    );
    bit_array_set(
        Some(&mut (*borrow_mut(&mut s).unwrap()).not_prime),
        1 as std::os::raw::c_int as uint32_t,
        1 as std::os::raw::c_int != 0,
    );
    let mut p: uint32_t = 2 as std::os::raw::c_int as uint32_t;
    while p.wrapping_mul(p) <= limit {
        if bit_array_get(borrow(& Some(&mut (*borrow_mut(&mut s).unwrap()).not_prime)), p) as std::os::raw::c_int == 0 as std::os::raw::c_int {
            let mut q: uint32_t = p.wrapping_mul(p);
            while q <= limit {
                bit_array_set(Some(&mut (*borrow_mut(&mut s).unwrap()).not_prime), q, 1 as std::os::raw::c_int != 0);
                q = (q as std::os::raw::c_uint).wrapping_add(p) as uint32_t as uint32_t;
            }
        }
        p = p.wrapping_add(1);
        p;
    }
    (*borrow_mut(&mut s).unwrap()).limit = limit;
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn sieve_destroy<'a1>(mut s: Option<&'a1 mut crate::sieve_tag>) {
    bit_array_destroy(Some(&mut (*borrow_mut(&mut s).unwrap()).not_prime));
}
#[no_mangle]
pub unsafe extern "C" fn is_prime<'a1>(mut s: Option<&'a1 crate::sieve_tag>, mut n: std::os::raw::c_uint) -> bool {
    if n <= (*(s).clone().unwrap()).limit {} else {
        __assert_fail(
            b"n <= s->limit\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            67 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 40],
                &[std::os::raw::c_char; 40],
            >(b"_Bool is_prime(const sieve *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_1919: {
        if n <= (*(s).clone().unwrap()).limit {} else {
            __assert_fail(
                b"n <= s->limit\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                67 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 40],
                    &[std::os::raw::c_char; 40],
                >(b"_Bool is_prime(const sieve *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return bit_array_get((Some(&(*(s).clone().unwrap()).not_prime)).clone(), n) as std::os::raw::c_int == 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn find_prime_partition<'a1>(
    mut s: Option<&'a1 crate::sieve_tag>,
    mut number: std::os::raw::c_uint,
    mut count: std::os::raw::c_uint,
    mut min_prime: std::os::raw::c_uint,
    mut p: * mut std::os::raw::c_uint,
) -> bool {
    if count == 1 as std::os::raw::c_int as std::os::raw::c_uint {
        if number >= min_prime && is_prime((s).clone(), number) as std::os::raw::c_int != 0 {
            *p = number;
            return 1 as std::os::raw::c_int != 0;
        }
        return 0 as std::os::raw::c_int != 0;
    }
    let mut prime: uint32_t = min_prime;
    while prime < number {
        if is_prime((s).clone(), prime) {
            if find_prime_partition(
                (s).clone(),
                number.wrapping_sub(prime),
                count.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint),
                prime.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint),
                p.offset(1 as std::os::raw::c_int as isize),
            ) {
                *p = prime;
                return 1 as std::os::raw::c_int != 0;
            }
        }
        prime = prime.wrapping_add(1);
        prime;
    }
    return 0 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn print_prime_partition<'a1>(
    mut s: Option<&'a1 crate::sieve_tag>,
    mut number: std::os::raw::c_uint,
    mut count: std::os::raw::c_uint,
) {
    if count > 0 as std::os::raw::c_int as std::os::raw::c_uint {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            93 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 62],
                &[std::os::raw::c_char; 62],
            >(b"void print_prime_partition(const sieve *, uint32_t, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_2193: {
        if count > 0 as std::os::raw::c_int as std::os::raw::c_uint {} else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                93 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 62],
                    &[std::os::raw::c_char; 62],
                >(b"void print_prime_partition(const sieve *, uint32_t, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut primes: *mut uint32_t = malloc(
        (count as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<uint32_t>() as std::os::raw::c_ulong),
    ) as *mut uint32_t;
    if primes.is_null() {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const std::os::raw::c_char);
        return;
    }
    if !find_prime_partition((s).clone(), number, count, 2 as std::os::raw::c_int as uint32_t, primes) {
        printf(
            b"%u cannot be partitioned into %u primes.\n\0" as *const u8
                as *const std::os::raw::c_char,
            number,
            count,
        );
    } else {
        printf(
            b"%u = %u\0" as *const u8 as *const std::os::raw::c_char,
            number,
            *primes.offset(0 as std::os::raw::c_int as isize),
        );
        let mut i: uint32_t = 1 as std::os::raw::c_int as uint32_t;
        while i < count {
            printf(
                b" + %u\0" as *const u8 as *const std::os::raw::c_char,
                *primes.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    }
    free(primes as *mut u8);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let limit: uint32_t = 100000 as std::os::raw::c_int as uint32_t;
    let mut s: sieve = {
        let mut init = sieve_tag {
            limit: 0 as std::os::raw::c_int as uint32_t,
            not_prime: bit_array {
                size: 0,
                array: core::ptr::null_mut(),
            },
        };
        init
    };
    if !sieve_create(Some(&mut s), limit) {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const std::os::raw::c_char);
        return 1 as std::os::raw::c_int;
    }
    print_prime_partition(
        borrow(& Some(&mut s)),
        99809 as std::os::raw::c_int as uint32_t,
        1 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        18 as std::os::raw::c_int as uint32_t,
        2 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        19 as std::os::raw::c_int as uint32_t,
        3 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        20 as std::os::raw::c_int as uint32_t,
        4 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        2017 as std::os::raw::c_int as uint32_t,
        24 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        22699 as std::os::raw::c_int as uint32_t,
        1 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        22699 as std::os::raw::c_int as uint32_t,
        2 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        22699 as std::os::raw::c_int as uint32_t,
        3 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        22699 as std::os::raw::c_int as uint32_t,
        4 as std::os::raw::c_int as uint32_t,
    );
    print_prime_partition(
        borrow(& Some(&mut s)),
        40355 as std::os::raw::c_int as uint32_t,
        3 as std::os::raw::c_int as uint32_t,
    );
    sieve_destroy(Some(&mut s));
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

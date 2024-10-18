#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn ceil(_: std::os::raw::c_double) -> std::os::raw::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
}
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
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
pub type llong_t = std::os::raw::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrimeArray {
    pub ptr: * mut std::os::raw::c_longlong,
    pub size: std::os::raw::c_ulong,
    pub capacity: std::os::raw::c_ulong,
}
impl std::default::Default for PrimeArray {
    fn default() -> Self {
        PrimeArray {
        ptr: 0 as * mut std::os::raw::c_longlong,
        size: std::os::raw::c_ulong::default(),
        capacity: std::os::raw::c_ulong::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn allocate() -> PrimeArray {
    let mut primes: PrimeArray = PrimeArray {
        ptr: 0 as *mut llong_t,
        size: 0,
        capacity: 0,
    };
    primes.size = 0 as std::os::raw::c_int as size_t;
    primes.capacity = 10 as std::os::raw::c_int as size_t;
    primes
        .ptr = malloc(
        (primes.capacity)
            .wrapping_mul(std::mem::size_of::<llong_t>() as std::os::raw::c_ulong),
    ) as *mut llong_t;
    return primes;
}
#[no_mangle]
pub unsafe extern "C" fn deallocate<'a1>(mut primes: Option<&'a1 mut crate::PrimeArray>) {
    free((*borrow_mut(&mut primes).unwrap()).ptr as *mut u8);
    (*borrow_mut(&mut primes).unwrap()).ptr = 0 as *mut llong_t;
}
#[no_mangle]
pub unsafe extern "C" fn push_back<'a1>(mut primes: Option<&'a1 mut crate::PrimeArray>, mut p: std::os::raw::c_longlong) {
    if (*borrow(& primes).unwrap()).size >= (*borrow(& primes).unwrap()).capacity {
        let mut new_capacity: size_t = (3 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul((*borrow_mut(&mut primes).unwrap()).capacity)
            .wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
        let mut temp: *mut llong_t = realloc(
            (*borrow_mut(&mut primes).unwrap()).ptr as *mut u8,
            new_capacity.wrapping_mul(std::mem::size_of::<llong_t>() as std::os::raw::c_ulong),
        ) as *mut llong_t;
        if temp.is_null() {
            fprintf(
                stderr,
                b"Failed to reallocate the prime array.\0" as *const u8
                    as *const std::os::raw::c_char,
            );
            exit(1 as std::os::raw::c_int);
        } else {
            (*borrow_mut(&mut primes).unwrap()).ptr = temp;
            (*borrow_mut(&mut primes).unwrap()).capacity = new_capacity;
        }
    }
    let fresh0 = (*borrow_mut(&mut primes).unwrap()).size;
    (*borrow_mut(&mut primes).unwrap()).size = ((*borrow_mut(&mut primes).unwrap()).size).wrapping_add(1);
    *((*borrow_mut(&mut primes).unwrap()).ptr).offset(fresh0 as isize) = p;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let cutOff: std::os::raw::c_int = 200 as std::os::raw::c_int;
    let bigUn: std::os::raw::c_int = 100000 as std::os::raw::c_int;
    let chunks: std::os::raw::c_int = 50 as std::os::raw::c_int;
    let little: std::os::raw::c_int = bigUn / chunks;
    let mut primes: PrimeArray = allocate();
    let mut c: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut showEach: bool = 1 as std::os::raw::c_int != 0;
    let mut u: llong_t = 0 as std::os::raw::c_int as llong_t;
    let mut v: llong_t = 1 as std::os::raw::c_int as llong_t;
    let mut i: llong_t = 0;
    push_back(Some(&mut primes), 3 as std::os::raw::c_int as llong_t);
    push_back(Some(&mut primes), 5 as std::os::raw::c_int as llong_t);
    printf(
        b"The first %d cuban primes:\n\0" as *const u8 as *const std::os::raw::c_char,
        cutOff,
    );
    i = 1 as std::os::raw::c_int as llong_t;
    while i < 9223372036854775807 as std::os::raw::c_longlong {
        let mut found: bool = 0 as std::os::raw::c_int != 0;
        u += 6 as std::os::raw::c_int as std::os::raw::c_longlong;
        v += u;
        let mut mx: llong_t = ceil(sqrt(v as std::os::raw::c_double)) as llong_t;
        let mut j: llong_t = 0;
        j = 0 as std::os::raw::c_int as llong_t;
        while (j as std::os::raw::c_ulonglong) < primes.size as std::os::raw::c_ulonglong {
            if *(primes.ptr).offset(j as isize) > mx {
                break;
            }
            if v % *(primes.ptr).offset(j as isize)
                == 0 as std::os::raw::c_int as std::os::raw::c_longlong
            {
                found = 1 as std::os::raw::c_int != 0;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if !found {
            c += 1 as std::os::raw::c_int;
            if showEach {
                let mut z: llong_t = 0;
                z = *(primes.ptr)
                    .offset(
                        (primes.size).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as isize,
                    ) + 2 as std::os::raw::c_int as std::os::raw::c_longlong;
                while z <= v - 2 as std::os::raw::c_int as std::os::raw::c_longlong {
                    let mut fnd: bool = 0 as std::os::raw::c_int != 0;
                    j = 0 as std::os::raw::c_int as llong_t;
                    while (j as std::os::raw::c_ulonglong) < primes.size as std::os::raw::c_ulonglong {
                        if *(primes.ptr).offset(j as isize) > mx {
                            break;
                        }
                        if z % *(primes.ptr).offset(j as isize)
                            == 0 as std::os::raw::c_int as std::os::raw::c_longlong
                        {
                            fnd = 1 as std::os::raw::c_int != 0;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if !fnd {
                        push_back(Some(&mut primes), z);
                    }
                    z += 2 as std::os::raw::c_int as std::os::raw::c_longlong;
                }
                push_back(Some(&mut primes), v);
                printf(b"%11lld\0" as *const u8 as *const std::os::raw::c_char, v);
                if c % 10 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
                }
                if c == cutOff {
                    showEach = 0 as std::os::raw::c_int != 0;
                    printf(
                        b"\nProgress to the %dth cuban prime: \0" as *const u8
                            as *const std::os::raw::c_char,
                        bigUn,
                    );
                }
            }
            if c % little == 0 as std::os::raw::c_int {
                printf(b".\0" as *const u8 as *const std::os::raw::c_char);
                if c == bigUn {
                    break;
                }
            }
        }
        i += 1;
        i;
    }
    printf(
        b"\nThe %dth cuban prime is %lld\n\0" as *const u8 as *const std::os::raw::c_char,
        c,
        v,
    );
    deallocate(Some(&mut primes));
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

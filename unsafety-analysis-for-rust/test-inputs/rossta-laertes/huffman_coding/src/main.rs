#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn abs(_: std::os::raw::c_int) -> std::os::raw::c_int;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffcode {
    pub nbits: std::os::raw::c_int,
    pub code: std::os::raw::c_int,
}
impl std::default::Default for huffcode {
    fn default() -> Self {
        huffcode {
        nbits: std::os::raw::c_int::default(),
        code: std::os::raw::c_int::default()
        }
    }
}

pub type huffcode_t = crate::huffcode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffheap {
    pub h: * mut std::os::raw::c_int,
    pub n: std::os::raw::c_int,
    pub s: std::os::raw::c_int,
    pub cs: std::os::raw::c_int,
    pub f: * mut std::os::raw::c_long,
}
impl std::default::Default for huffheap {
    fn default() -> Self {
        huffheap {
        h: 0 as * mut std::os::raw::c_int,
        n: std::os::raw::c_int::default(),
        s: std::os::raw::c_int::default(),
        cs: std::os::raw::c_int::default(),
        f: 0 as * mut std::os::raw::c_long
        }
    }
}

pub type heap_t = crate::huffheap;
unsafe extern "C" fn _heap_create(
    mut s: std::os::raw::c_int,
    mut f: *mut std::os::raw::c_long,
) -> *mut heap_t {
    let mut h: *mut heap_t = 0 as *mut heap_t;
    h = malloc(std::mem::size_of::<heap_t>() as std::os::raw::c_ulong) as *mut heap_t;
    (*h)
        .h = malloc(
        (std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(s as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    (*h).cs = s;
    (*h).s = (*h).cs;
    (*h).n = 0 as std::os::raw::c_int;
    (*h).f = f;
    return h;
}
unsafe extern "C" fn _heap_destroy(mut heap: *mut heap_t) {
    free((*heap).h as *mut u8);
    free(heap as *mut u8);
}
unsafe extern "C" fn _heap_sort(mut heap: *mut heap_t) {
    let mut i: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut j: std::os::raw::c_int = 2 as std::os::raw::c_int;
    let mut a: *mut std::os::raw::c_int = (*heap).h;
    while i < (*heap).n {
        if *((*heap).f).offset(*a.offset((i - 1 as std::os::raw::c_int) as isize) as isize)
            >= *((*heap).f).offset(*a.offset(i as isize) as isize)
        {
            i = j;
            j += 1;
            j;
        } else {
            let mut t_: std::os::raw::c_int = 0;
            t_ = *a.offset((i - 1 as std::os::raw::c_int) as isize);
            *a.offset((i - 1 as std::os::raw::c_int) as isize) = *a.offset(i as isize);
            *a.offset(i as isize) = t_;
            i -= 1;
            i;
            i = if i == 0 as std::os::raw::c_int {
                let fresh0 = j;
                j = j + 1;
                fresh0
            } else {
                i
            };
        }
    }
}
unsafe extern "C" fn _heap_add(mut heap: *mut heap_t, mut c: std::os::raw::c_int) {
    if (*heap).n + 1 as std::os::raw::c_int > (*heap).s {
        (*heap)
            .h = realloc(
            (*heap).h as *mut u8,
            ((*heap).s + (*heap).cs) as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_int;
        (*heap).s += (*heap).cs;
    }
    *((*heap).h).offset((*heap).n as isize) = c;
    (*heap).n += 1;
    (*heap).n;
    _heap_sort(heap);
}
unsafe extern "C" fn _heap_remove(mut heap: *mut heap_t) -> std::os::raw::c_int {
    if (*heap).n > 0 as std::os::raw::c_int {
        (*heap).n -= 1;
        (*heap).n;
        return *((*heap).h).offset((*heap).n as isize);
    }
    return -(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn create_huffman_codes(
    mut freqs: *mut std::os::raw::c_long,
) -> *mut *mut huffcode_t {
    let mut codes: *mut *mut huffcode_t = 0 as *mut *mut huffcode_t;
    let mut heap: *mut heap_t = 0 as *mut heap_t;
    let mut efreqs: [std::os::raw::c_long; 512] = [0; 512];
    let mut preds: [std::os::raw::c_int; 512] = [0; 512];
    let mut i: std::os::raw::c_int = 0;
    let mut extf: std::os::raw::c_int = 256 as std::os::raw::c_int;
    let mut r1: std::os::raw::c_int = 0;
    let mut r2: std::os::raw::c_int = 0;
    memcpy(
        efreqs.as_mut_ptr() as *mut u8,
        freqs as *const u8,
        (std::mem::size_of::<std::os::raw::c_long>() as std::os::raw::c_ulong)
            .wrapping_mul(256 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    memset(
        &mut *efreqs.as_mut_ptr().offset(256 as std::os::raw::c_int as isize)
            as *mut std::os::raw::c_long as *mut u8,
        0 as std::os::raw::c_int,
        (std::mem::size_of::<std::os::raw::c_long>() as std::os::raw::c_ulong)
            .wrapping_mul(256 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    heap = _heap_create(256 as std::os::raw::c_int * 2 as std::os::raw::c_int, efreqs.as_mut_ptr());
    if heap.is_null() {
        return 0 as *mut *mut huffcode_t;
    }
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        if efreqs[i as usize] > 0 as std::os::raw::c_int as std::os::raw::c_long {
            _heap_add(heap, i);
        }
        i += 1;
        i;
    }
    while (*heap).n > 1 as std::os::raw::c_int {
        r1 = _heap_remove(heap);
        r2 = _heap_remove(heap);
        efreqs[extf as usize] = efreqs[r1 as usize] + efreqs[r2 as usize];
        _heap_add(heap, extf);
        preds[r1 as usize] = extf;
        preds[r2 as usize] = -extf;
        extf += 1;
        extf;
    }
    r1 = _heap_remove(heap);
    preds[r1 as usize] = r1;
    _heap_destroy(heap);
    codes = malloc(
        (std::mem::size_of::<*mut huffcode_t>() as std::os::raw::c_ulong)
            .wrapping_mul(256 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut *mut huffcode_t;
    let mut bc: std::os::raw::c_int = 0;
    let mut bn: std::os::raw::c_int = 0;
    let mut ix: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        bc = 0 as std::os::raw::c_int;
        bn = 0 as std::os::raw::c_int;
        if efreqs[i as usize] == 0 as std::os::raw::c_int as std::os::raw::c_long {
            let ref mut fresh1 = *codes.offset(i as isize);
            *fresh1 = 0 as *mut huffcode_t;
        } else {
            ix = i;
            while abs(preds[ix as usize]) != ix {
                bc
                    |= (if preds[ix as usize] >= 0 as std::os::raw::c_int {
                        1 as std::os::raw::c_int
                    } else {
                        0 as std::os::raw::c_int
                    }) << bn;
                ix = abs(preds[ix as usize]);
                bn += 1;
                bn;
            }
            let ref mut fresh2 = *codes.offset(i as isize);
            *fresh2 = malloc(std::mem::size_of::<huffcode_t>() as std::os::raw::c_ulong)
                as *mut huffcode_t;
            (**codes.offset(i as isize)).nbits = bn;
            (**codes.offset(i as isize)).code = bc;
        }
        i += 1;
        i;
    }
    return codes;
}
#[no_mangle]
pub unsafe extern "C" fn free_huffman_codes(mut c: *mut *mut huffcode_t) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        free(*c.offset(i as isize) as *mut u8);
        i += 1;
        i;
    }
    free(c as *mut u8);
}
#[no_mangle]
pub unsafe extern "C" fn inttobits(
    mut c: std::os::raw::c_int,
    mut n: std::os::raw::c_int,
    mut s: *mut std::os::raw::c_char,
) {
    *s.offset(n as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    while n > 0 as std::os::raw::c_int {
        *s
            .offset(
                (n - 1 as std::os::raw::c_int) as isize,
            ) = (c % 2 as std::os::raw::c_int + '0' as i32) as std::os::raw::c_char;
        c >>= 1 as std::os::raw::c_int;
        n -= 1;
        n;
    }
}
#[no_mangle]
pub static mut test: *const std::os::raw::c_char = b"this is an example for huffman encoding\0"
    as *const u8 as *const std::os::raw::c_char;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut r: *mut *mut huffcode_t = 0 as *mut *mut huffcode_t;
    let mut i: std::os::raw::c_int = 0;
    let mut strbit: [std::os::raw::c_char; 100] = [0; 100];
    let mut p: *const std::os::raw::c_char = core::ptr::null();
    let mut freqs: [std::os::raw::c_long; 256] = [0; 256];
    memset(
        freqs.as_mut_ptr() as *mut u8,
        0 as std::os::raw::c_int,
        std::mem::size_of::<[std::os::raw::c_long; 256]>() as std::os::raw::c_ulong,
    );
    p = test;
    while *p as std::os::raw::c_int != '\0' as i32 {
        let fresh3 = p;
        p = p.offset(1);
        freqs[*fresh3 as usize] += 1;
        freqs[*fresh3 as usize];
    }
    r = create_huffman_codes(freqs.as_mut_ptr());
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        if !(*r.offset(i as isize)).is_null() {
            inttobits(
                (**r.offset(i as isize)).code,
                (**r.offset(i as isize)).nbits,
                strbit.as_mut_ptr(),
            );
            printf(
                b"%c (%d) %s\n\0" as *const u8 as *const std::os::raw::c_char,
                i,
                (**r.offset(i as isize)).code,
                strbit.as_mut_ptr(),
            );
        }
        i += 1;
        i;
    }
    free_huffman_codes(r);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

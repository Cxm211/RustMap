#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn rand() -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
pub type size_t = std::os::raw::c_ulong;
pub type DATA = std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_t {
    pub buf: * mut std::os::raw::c_int,
    pub head: std::os::raw::c_ulong,
    pub tail: std::os::raw::c_ulong,
    pub alloc: std::os::raw::c_ulong,
}
impl std::default::Default for queue_t {
    fn default() -> Self {
        queue_t {
        buf: 0 as * mut std::os::raw::c_int,
        head: std::os::raw::c_ulong::default(),
        tail: std::os::raw::c_ulong::default(),
        alloc: std::os::raw::c_ulong::default()
        }
    }
}

pub type queue = * mut crate::queue_t;
#[no_mangle]
pub unsafe extern "C" fn q_new() -> queue {
    let mut q: queue = malloc(std::mem::size_of::<queue_t>() as std::os::raw::c_ulong)
        as queue;
    (*q).alloc = 4 as std::os::raw::c_int as size_t;
    (*q)
        .buf = malloc(
        (std::mem::size_of::<DATA>() as std::os::raw::c_ulong).wrapping_mul((*q).alloc),
    ) as *mut DATA;
    (*q).tail = 0 as std::os::raw::c_int as size_t;
    (*q).head = (*q).tail;
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn empty<'a1>(mut q: Option<&'a1 mut crate::queue_t>) -> std::os::raw::c_int {
    return ((*borrow(& q).unwrap()).tail == (*borrow(& q).unwrap()).head) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enqueue(mut q: queue, mut n: DATA) {
    if (*q).tail >= (*q).alloc {
        (*q).tail = 0 as std::os::raw::c_int as size_t;
    }
    let fresh0 = (*q).tail;
    (*q).tail = ((*q).tail).wrapping_add(1);
    *((*q).buf).offset(fresh0 as isize) = n;
    if (*q).tail == (*q).alloc {
        (*q)
            .buf = realloc(
            (*q).buf as *mut u8,
            (std::mem::size_of::<DATA>() as std::os::raw::c_ulong)
                .wrapping_mul((*q).alloc)
                .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong),
        ) as *mut DATA;
        if (*q).head != 0 {
            memcpy(
                ((*q).buf).offset((*q).head as isize).offset((*q).alloc as isize)
                    as *mut u8,
                ((*q).buf).offset((*q).head as isize) as *const u8,
                (std::mem::size_of::<DATA>() as std::os::raw::c_ulong)
                    .wrapping_mul(((*q).alloc).wrapping_sub((*q).head)),
            );
            (*q)
                .head = ((*q).head as std::os::raw::c_ulong).wrapping_add((*q).alloc) as size_t
                as size_t;
        } else {
            (*q).tail = (*q).alloc;
        }
        (*q)
            .alloc = ((*q).alloc as std::os::raw::c_ulong)
            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dequeue<'a1>(mut q: * mut crate::queue_t, mut n: Option<&'a1 mut std::os::raw::c_int>) -> std::os::raw::c_int {
    if (*q).head == (*q).tail {
        return 0 as std::os::raw::c_int;
    }
    let fresh1 = (*q).head;
    (*q).head = ((*q).head).wrapping_add(1);
    *borrow_mut(&mut n).unwrap() = *((*q).buf).offset(fresh1 as isize);
    if (*q).head >= (*q).alloc {
        (*q).head = 0 as std::os::raw::c_int as size_t;
        if (*q).alloc >= 512 as std::os::raw::c_int as std::os::raw::c_ulong
            && (*q).tail < ((*q).alloc).wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        {
            (*q)
                .alloc = ((*q).alloc as std::os::raw::c_ulong)
                .wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            (*q)
                .buf = realloc(
                (*q).buf as *mut u8,
                (std::mem::size_of::<DATA>() as std::os::raw::c_ulong)
                    .wrapping_mul((*q).alloc),
            ) as *mut DATA;
        }
    }
    return 1 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut q: queue = q_new();
    i = 0 as std::os::raw::c_int;
    while i < 100000000 as std::os::raw::c_int {
        n = rand();
        if n > 2147483647 as std::os::raw::c_int / 2 as std::os::raw::c_int {
            enqueue(q, n);
        } else {
            dequeue(q, Some(&mut n)) == 0;
        }
        i += 1;
        i;
    }
    while dequeue(q, Some(&mut n)) != 0 {}
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

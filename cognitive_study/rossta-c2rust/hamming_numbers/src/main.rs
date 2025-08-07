#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
}
pub type size_t = std::os::raw::c_ulong;
pub type ham = std::os::raw::c_ulonglong;
#[no_mangle]
pub static mut alloc: size_t = 0 as std::os::raw::c_int as size_t;
#[no_mangle]
pub static mut n: size_t = 1 as std::os::raw::c_int as size_t;
#[no_mangle]
pub static mut q: *mut ham = 0 as *const ham as *mut ham;
#[no_mangle]
pub unsafe extern "C" fn qpush(mut h: ham) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    if alloc <= n {
        alloc = if alloc != 0 {
            alloc.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        } else {
            16 as std::os::raw::c_int as std::os::raw::c_ulong
        };
        q = realloc(
            q as *mut std::os::raw::c_void,
            (::core::mem::size_of::<ham>() as std::os::raw::c_ulong).wrapping_mul(alloc),
        ) as *mut ham;
    }
    let fresh0 = n;
    n = n.wrapping_add(1);
    i = fresh0 as std::os::raw::c_int;
    loop {
        j = i / 2 as std::os::raw::c_int;
        if !(j != 0 && *q.offset(j as isize) > h) {
            break;
        }
        *q.offset(i as isize) = *q.offset(j as isize);
        i = j;
    }
    *q.offset(i as isize) = h;
}
#[no_mangle]
pub unsafe extern "C" fn qpop() -> ham {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut r: ham = 0;
    let mut t: ham = 0;
    r = *q.offset(1 as std::os::raw::c_int as isize);
    while n > 1 as std::os::raw::c_int as std::os::raw::c_ulong
        && r == *q.offset(1 as std::os::raw::c_int as isize)
    {
        i = 1 as std::os::raw::c_int;
        n = n.wrapping_sub(1);
        t = *q.offset(n as isize);
        loop {
            j = i * 2 as std::os::raw::c_int;
            if !((j as std::os::raw::c_ulong) < n) {
                break;
            }
            if ((j + 1 as std::os::raw::c_int) as std::os::raw::c_ulong) < n
                && *q.offset(j as isize) > *q.offset((j + 1 as std::os::raw::c_int) as isize)
            {
                j += 1;
                j;
            }
            if t <= *q.offset(j as isize) {
                break;
            }
            *q.offset(i as isize) = *q.offset(j as isize);
            i = j;
        }
        *q.offset(i as isize) = t;
    }
    return r;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut h: ham = 0;
    i = 1 as std::os::raw::c_int;
    qpush(i as ham);
    while i <= 1691 as std::os::raw::c_int {
        h = qpop();
        qpush(h.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulonglong));
        qpush(h.wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulonglong));
        qpush(h.wrapping_mul(5 as std::os::raw::c_int as std::os::raw::c_ulonglong));
        if i <= 20 as std::os::raw::c_int || i == 1691 as std::os::raw::c_int {
            printf(b"%6d: %llu\n\0" as *const u8 as *const std::os::raw::c_char, i, h);
        }
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

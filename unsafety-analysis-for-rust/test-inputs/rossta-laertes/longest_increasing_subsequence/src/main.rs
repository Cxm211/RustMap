#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub val: std::os::raw::c_int,
    pub len: std::os::raw::c_int,
    pub next: * mut crate::node,
}
impl std::default::Default for node {
    fn default() -> Self {
        node {
        val: std::os::raw::c_int::default(),
        len: std::os::raw::c_int::default(),
        next: core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn lis(mut v: *mut std::os::raw::c_int, mut len: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut p: *mut node = core::ptr::null_mut();
    let mut n: *mut node = calloc(
        len as std::os::raw::c_ulong,
        std::mem::size_of::<node>() as std::os::raw::c_ulong,
    ) as *mut node;
    i = 0 as std::os::raw::c_int;
    while i < len {
        (*n.offset(i as isize)).val = *v.offset(i as isize);
        i += 1;
        i;
    }
    i = len;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        p = n.offset(i as isize);
        loop {
            let fresh1 = p;
            p = p.offset(1);
            if !(fresh1 < n.offset(len as isize)) {
                break;
            }
            if (*p).val > (*n.offset(i as isize)).val
                && (*p).len >= (*n.offset(i as isize)).len
            {
                let ref mut fresh2 = (*n.offset(i as isize)).next;
                *fresh2 = p;
                (*n.offset(i as isize)).len = (*p).len + 1 as std::os::raw::c_int;
            }
        }
    }
    i = 0 as std::os::raw::c_int;
    p = n;
    while i < len {
        if (*n.offset(i as isize)).len > (*p).len {
            p = n.offset(i as isize);
        }
        i += 1;
        i;
    }
    loop {
        printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, (*p).val);
        p = (*p).next;
        if p.is_null() {
            break;
        }
    }
    putchar('\n' as i32);
    free(n as *mut u8);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: [std::os::raw::c_int; 6] = [
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        6 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    ];
    let mut y: [std::os::raw::c_int; 16] = [
        0 as std::os::raw::c_int,
        8 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        12 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        10 as std::os::raw::c_int,
        6 as std::os::raw::c_int,
        14 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        9 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        13 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        11 as std::os::raw::c_int,
        7 as std::os::raw::c_int,
        15 as std::os::raw::c_int,
    ];
    lis(
        x.as_mut_ptr(),
        (std::mem::size_of::<[std::os::raw::c_int; 6]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
    );
    lis(
        y.as_mut_ptr(),
        (std::mem::size_of::<[std::os::raw::c_int; 16]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

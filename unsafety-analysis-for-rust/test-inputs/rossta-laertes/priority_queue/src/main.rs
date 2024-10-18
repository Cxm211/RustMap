#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub priority: std::os::raw::c_int,
    pub data: * mut std::os::raw::c_char,
}
impl std::default::Default for node_t {
    fn default() -> Self {
        node_t {
        priority: std::os::raw::c_int::default(),
        data: 0 as * mut std::os::raw::c_char
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_t {
    pub nodes: * mut crate::node_t,
    pub len: std::os::raw::c_int,
    pub size: std::os::raw::c_int,
}
impl std::default::Default for heap_t {
    fn default() -> Self {
        heap_t {
        nodes: core::ptr::null_mut(),
        len: std::os::raw::c_int::default(),
        size: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn push(
    mut h: *mut heap_t,
    mut priority: std::os::raw::c_int,
    mut data: *mut std::os::raw::c_char,
) {
    if (*h).len + 1 as std::os::raw::c_int >= (*h).size {
        (*h)
            .size = if (*h).size != 0 {
            (*h).size * 2 as std::os::raw::c_int
        } else {
            4 as std::os::raw::c_int
        };
        (*h)
            .nodes = realloc(
            (*h).nodes as *mut u8,
            ((*h).size as std::os::raw::c_ulong)
                .wrapping_mul(std::mem::size_of::<node_t>() as std::os::raw::c_ulong),
        ) as *mut node_t;
    }
    let mut i: std::os::raw::c_int = (*h).len + 1 as std::os::raw::c_int;
    let mut j: std::os::raw::c_int = i / 2 as std::os::raw::c_int;
    while i > 1 as std::os::raw::c_int && (*((*h).nodes).offset(j as isize)).priority > priority
    {
        *((*h).nodes).offset(i as isize) = *((*h).nodes).offset(j as isize);
        i = j;
        j = j / 2 as std::os::raw::c_int;
    }
    (*((*h).nodes).offset(i as isize)).priority = priority;
    let ref mut fresh0 = (*((*h).nodes).offset(i as isize)).data;
    *fresh0 = data;
    (*h).len += 1;
    (*h).len;
}
#[no_mangle]
pub unsafe extern "C" fn pop(mut h: *mut heap_t) -> *mut std::os::raw::c_char {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    if (*h).len == 0 {
        return 0 as *mut std::os::raw::c_char;
    }
    let mut data: *mut std::os::raw::c_char = (*((*h).nodes).offset(1 as std::os::raw::c_int as isize))
        .data;
    *((*h).nodes)
        .offset(1 as std::os::raw::c_int as isize) = *((*h).nodes).offset((*h).len as isize);
    (*h).len -= 1;
    (*h).len;
    i = 1 as std::os::raw::c_int;
    while i != (*h).len + 1 as std::os::raw::c_int {
        k = (*h).len + 1 as std::os::raw::c_int;
        j = 2 as std::os::raw::c_int * i;
        if j <= (*h).len
            && (*((*h).nodes).offset(j as isize)).priority
                < (*((*h).nodes).offset(k as isize)).priority
        {
            k = j;
        }
        if j + 1 as std::os::raw::c_int <= (*h).len
            && (*((*h).nodes).offset((j + 1 as std::os::raw::c_int) as isize)).priority
                < (*((*h).nodes).offset(k as isize)).priority
        {
            k = j + 1 as std::os::raw::c_int;
        }
        *((*h).nodes).offset(i as isize) = *((*h).nodes).offset(k as isize);
        i = k;
    }
    return data;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut h: *mut heap_t = calloc(
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        std::mem::size_of::<heap_t>() as std::os::raw::c_ulong,
    ) as *mut heap_t;
    push(
        h,
        3 as std::os::raw::c_int,
        b"Clear drains\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    );
    push(
        h,
        4 as std::os::raw::c_int,
        b"Feed cat\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    );
    push(
        h,
        5 as std::os::raw::c_int,
        b"Make tea\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    );
    push(
        h,
        1 as std::os::raw::c_int,
        b"Solve RC tasks\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    );
    push(
        h,
        2 as std::os::raw::c_int,
        b"Tax return\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    );
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, pop(h));
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

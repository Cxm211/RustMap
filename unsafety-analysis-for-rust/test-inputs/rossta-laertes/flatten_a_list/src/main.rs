#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strtol(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_long;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub is_list: std::os::raw::c_int,
    pub ival: std::os::raw::c_int,
    pub lst: * mut * mut crate::list_t,
}
impl std::default::Default for list_t {
    fn default() -> Self {
        list_t {
        is_list: std::os::raw::c_int::default(),
        ival: std::os::raw::c_int::default(),
        lst: core::ptr::null_mut()
        }
    }
}

pub type list = * mut crate::list_t;
#[no_mangle]
pub unsafe extern "C" fn new_list() -> list {
    let mut x: list = malloc(std::mem::size_of::<list_t>() as std::os::raw::c_ulong) as list;
    (*x).ival = 0 as std::os::raw::c_int;
    (*x).is_list = 1 as std::os::raw::c_int;
    (*x).lst = 0 as *mut list;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn append(mut parent: list, mut child: list) {
    (*parent)
        .lst = realloc(
        (*parent).lst as *mut u8,
        (std::mem::size_of::<list>() as std::os::raw::c_ulong)
            .wrapping_mul(((*parent).ival + 1 as std::os::raw::c_int) as std::os::raw::c_ulong),
    ) as *mut list;
    let fresh0 = (*parent).ival;
    (*parent).ival = (*parent).ival + 1;
    let ref mut fresh1 = *((*parent).lst).offset(fresh0 as isize);
    *fresh1 = child;
}
#[no_mangle]
pub unsafe extern "C" fn from_string(
    mut s: *mut std::os::raw::c_char,
    mut e: *mut *mut std::os::raw::c_char,
    mut parent: list,
) -> list {
    let mut ret: list = 0 as list;
    if parent.is_null() {
        parent = new_list();
    }
    while *s as std::os::raw::c_int != '\0' as i32 {
        if *s as std::os::raw::c_int == ']' as i32 {
            if !e.is_null() {
                *e = s.offset(1 as std::os::raw::c_int as isize);
            }
            return parent;
        }
        if *s as std::os::raw::c_int == '[' as i32 {
            ret = new_list();
            (*ret).is_list = 1 as std::os::raw::c_int;
            (*ret).ival = 0 as std::os::raw::c_int;
            append(parent, ret);
            from_string(s.offset(1 as std::os::raw::c_int as isize), &mut s, ret);
        } else if *s as std::os::raw::c_int >= '0' as i32 && *s as std::os::raw::c_int <= '9' as i32 {
            ret = new_list();
            (*ret).is_list = 0 as std::os::raw::c_int;
            (*ret).ival = strtol(s, &mut s, 10 as std::os::raw::c_int) as std::os::raw::c_int;
            append(parent, ret);
        } else {
            s = s.offset(1);
            s;
        }
    }
    if !e.is_null() {
        *e = s;
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn show_list(mut l: list) {
    let mut i: std::os::raw::c_int = 0;
    if l.is_null() {
        return;
    }
    if (*l).is_list == 0 {
        printf(b"%d\0" as *const u8 as *const std::os::raw::c_char, (*l).ival);
        return;
    }
    printf(b"[\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < (*l).ival {
        show_list(*((*l).lst).offset(i as isize));
        if i < (*l).ival - 1 as std::os::raw::c_int {
            printf(b", \0" as *const u8 as *const std::os::raw::c_char);
        }
        i += 1;
        i;
    }
    printf(b"]\0" as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn flatten(mut from: list, mut to: list) -> list {
    let mut i: std::os::raw::c_int = 0;
    let mut t: list = 0 as *mut list_t;
    if to.is_null() {
        to = new_list();
    }
    if (*from).is_list == 0 {
        t = new_list();
        *t = *from;
        append(to, t);
    } else {
        i = 0 as std::os::raw::c_int;
        while i < (*from).ival {
            flatten(*((*from).lst).offset(i as isize), to);
            i += 1;
            i;
        }
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn delete_list(mut l: list) {
    let mut i: std::os::raw::c_int = 0;
    if l.is_null() {
        return;
    }
    if (*l).is_list != 0 && (*l).ival != 0 {
        i = 0 as std::os::raw::c_int;
        while i < (*l).ival {
            delete_list(*((*l).lst).offset(i as isize));
            i += 1;
            i;
        }
        free((*l).lst as *mut u8);
    }
    free(l as *mut u8);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut l: list = from_string(
        b"[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []\0" as *const u8
            as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        core::ptr::null_mut(),
        0 as list,
    );
    printf(b"Nested: \0" as *const u8 as *const std::os::raw::c_char);
    show_list(l);
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    let mut flat: list = flatten(l, 0 as list);
    printf(b"Flattened: \0" as *const u8 as *const std::os::raw::c_char);
    show_list(flat);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

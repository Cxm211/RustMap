#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn rand() -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_double_t {
    pub left: * mut crate::node_double_t,
    pub right: * mut crate::node_double_t,
    pub value: std::os::raw::c_double,
}
impl std::default::Default for node_double_t {
    fn default() -> Self {
        node_double_t {
        left: core::ptr::null_mut(),
        right: core::ptr::null_mut(),
        value: std::os::raw::c_double::default()
        }
    }
}

pub type node_double = * mut crate::node_double_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_int_t {
    pub left: * mut crate::node_int_t,
    pub right: * mut crate::node_int_t,
    pub value: std::os::raw::c_int,
}
impl std::default::Default for node_int_t {
    fn default() -> Self {
        node_int_t {
        left: core::ptr::null_mut(),
        right: core::ptr::null_mut(),
        value: std::os::raw::c_int::default()
        }
    }
}

pub type node_int = * mut crate::node_int_t;
#[no_mangle]
pub unsafe extern "C" fn node_double_new(mut v: std::os::raw::c_double) -> node_double {
    let mut node: node_double = malloc(
        std::mem::size_of::<node_double_t>() as std::os::raw::c_ulong,
    ) as node_double;
    (*node).value = v;
    (*node).right = core::ptr::null_mut();
    (*node).left = (*node).right;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn node_double_insert(
    mut root: node_double,
    mut v: std::os::raw::c_double,
) -> node_double {
    let mut n: node_double = node_double_new(v);
    while !root.is_null() {
        if (*root).value < (*n).value {
            if ((*root).left).is_null() {
                (*root).left = n;
                return (*root).left;
            } else {
                root = (*root).left;
            }
        } else if ((*root).right).is_null() {
            (*root).right = n;
            return (*root).right;
        } else {
            root = (*root).right;
        }
    }
    return core::ptr::null_mut();
}
#[no_mangle]
pub unsafe extern "C" fn node_int_new(mut v: std::os::raw::c_int) -> node_int {
    let mut node: node_int = malloc(
        std::mem::size_of::<node_int_t>() as std::os::raw::c_ulong,
    ) as node_int;
    (*node).value = v;
    (*node).right = core::ptr::null_mut();
    (*node).left = (*node).right;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn node_int_insert(
    mut root: node_int,
    mut v: std::os::raw::c_int,
) -> node_int {
    let mut n: node_int = node_int_new(v);
    while !root.is_null() {
        if (*root).value < (*n).value {
            if ((*root).left).is_null() {
                (*root).left = n;
                return (*root).left;
            } else {
                root = (*root).left;
            }
        } else if ((*root).right).is_null() {
            (*root).right = n;
            return (*root).right;
        } else {
            root = (*root).right;
        }
    }
    return core::ptr::null_mut();
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut root_d: node_double = node_double_new(
        rand() as std::os::raw::c_double / 2147483647 as std::os::raw::c_int as std::os::raw::c_double,
    );
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        node_double_insert(
            root_d,
            rand() as std::os::raw::c_double / 2147483647 as std::os::raw::c_int as std::os::raw::c_double,
        );
        i += 1;
        i;
    }
    let mut root_i: node_int = node_int_new(rand());
    i = 0 as std::os::raw::c_int;
    while i < 10000 as std::os::raw::c_int {
        node_int_insert(root_i, rand());
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

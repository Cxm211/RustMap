#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub s: * mut std::os::raw::c_char,
    pub prev: * mut crate::node,
}
impl std::default::Default for node {
    fn default() -> Self {
        node {
        s: 0 as * mut std::os::raw::c_char,
        prev: core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn powerset(
    mut v: *mut *mut std::os::raw::c_char,
    mut n: std::os::raw::c_int,
    mut up: *mut node,
) {
    let mut me: node = node {
        s: 0 as *mut std::os::raw::c_char,
        prev: core::ptr::null_mut(),
    };
    if n == 0 {
        putchar('[' as i32);
        while !up.is_null() {
            printf(b" %s\0" as *const u8 as *const std::os::raw::c_char, (*up).s);
            up = (*up).prev;
        }
        puts(b" ]\0" as *const u8 as *const std::os::raw::c_char);
    } else {
        me.s = *v;
        me.prev = up;
        powerset(v.offset(1 as std::os::raw::c_int as isize), n - 1 as std::os::raw::c_int, up);
        powerset(v.offset(1 as std::os::raw::c_int as isize), n - 1 as std::os::raw::c_int, &mut me);
    };
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    powerset(
        argv.offset(1 as std::os::raw::c_int as isize),
        argc - 1 as std::os::raw::c_int,
        core::ptr::null_mut(),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

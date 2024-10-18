#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn memcpy(
        _: *mut u8,
        _: *const u8,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = std::os::raw::c_ulong;
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_env {
    pub n: std::os::raw::c_uint,
    pub i: std::os::raw::c_uint,
    pub size: std::os::raw::c_ulong,
    pub sample: * mut u8,
}
impl std::default::Default for s_env {
    fn default() -> Self {
        s_env {
        n: std::os::raw::c_uint::default(),
        i: std::os::raw::c_uint::default(),
        size: std::os::raw::c_ulong::default(),
        sample: 0 as * mut u8
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn s_of_n_init<'a1>(
    mut s_env: Option<&'a1 mut crate::s_env>,
    mut size: std::os::raw::c_ulong,
    mut n: std::os::raw::c_uint,
) {
    (*borrow_mut(&mut s_env).unwrap()).i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*borrow_mut(&mut s_env).unwrap()).n = n;
    (*borrow_mut(&mut s_env).unwrap()).size = size;
    (*borrow_mut(&mut s_env).unwrap()).sample = malloc((n as std::os::raw::c_ulong).wrapping_mul(size));
}
#[no_mangle]
pub unsafe extern "C" fn sample_set_i<'a1>(
    mut s_env: Option<&'a1 mut crate::s_env>,
    mut i: std::os::raw::c_uint,
    mut item: * mut u8,
) {
    memcpy(
        ((*borrow_mut(&mut s_env).unwrap()).sample)
            .offset((i as std::os::raw::c_ulong).wrapping_mul((*borrow_mut(&mut s_env).unwrap()).size) as isize),
        item,
        (*borrow_mut(&mut s_env).unwrap()).size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn s_of_n<'a1>(
    mut s_env: Option<&'a1 mut crate::s_env>,
    mut item: * mut u8,
) -> * mut u8 {
    (*borrow_mut(&mut s_env).unwrap()).i = ((*borrow_mut(&mut s_env).unwrap()).i).wrapping_add(1);
    (*borrow(& s_env).unwrap()).i;
    if (*borrow(& s_env).unwrap()).i <= (*borrow(& s_env).unwrap()).n {
        sample_set_i(
            borrow_mut(&mut s_env),
            ((*borrow_mut(&mut s_env).unwrap()).i).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint),
            item,
        );
    } else if (rand() as std::os::raw::c_uint).wrapping_rem((*borrow_mut(&mut s_env).unwrap()).i) < (*borrow(& s_env).unwrap()).n {
        sample_set_i(borrow_mut(&mut s_env), (rand() as std::os::raw::c_uint).wrapping_rem((*borrow_mut(&mut s_env).unwrap()).n), item);
    }
    return (*borrow_mut(&mut s_env).unwrap()).sample;
}
#[no_mangle]
pub unsafe extern "C" fn test(
    mut n: std::os::raw::c_uint,
    mut items_set: *mut std::os::raw::c_int,
    mut num_items: std::os::raw::c_uint,
) -> *mut std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut s_env: s_env = s_env {
        n: 0,
        i: 0,
        size: 0,
        sample: 0 as *mut u8,
    };
    s_of_n_init(Some(&mut s_env), std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong, n);
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_uint) < num_items {
        s_of_n(
            Some(&mut s_env),
            &mut *items_set.offset(i as isize) as *mut std::os::raw::c_int as *mut u8,
        );
        i += 1;
        i;
    }
    return s_env.sample as *mut std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_uint = 0;
    let mut j: std::os::raw::c_uint = 0;
    let mut n: std::os::raw::c_uint = 3 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut num_items: std::os::raw::c_uint = 10 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut frequencies: *mut std::os::raw::c_uint = core::ptr::null_mut();
    let mut items_set: *mut std::os::raw::c_int = core::ptr::null_mut();
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    items_set = malloc(
        (num_items as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    frequencies = malloc(
        (num_items as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < num_items {
        *items_set.offset(i as isize) = i as std::os::raw::c_int;
        *frequencies.offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < 100000 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut res: *mut std::os::raw::c_int = test(n, items_set, num_items);
        j = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while j < n {
            let ref mut fresh0 = *frequencies.offset(*res.offset(j as isize) as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            *fresh0;
            j = j.wrapping_add(1);
            j;
        }
        free(res as *mut u8);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < num_items {
        printf(
            b" %d\0" as *const u8 as *const std::os::raw::c_char,
            *frequencies.offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    puts(b"\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

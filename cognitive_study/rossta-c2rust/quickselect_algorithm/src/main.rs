#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn qselect(
    mut v: *mut std::os::raw::c_int,
    mut len: std::os::raw::c_int,
    mut k: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut st: std::os::raw::c_int = 0;
    let mut tmp: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    st = i;
    while i < len - 1 as std::os::raw::c_int {
        if !(*v.offset(i as isize) > *v.offset((len - 1 as std::os::raw::c_int) as isize)) {
            tmp = *v.offset(i as isize);
            *v.offset(i as isize) = *v.offset(st as isize);
            *v.offset(st as isize) = tmp;
            st += 1;
            st;
        }
        i += 1;
        i;
    }
    tmp = *v.offset((len - 1 as std::os::raw::c_int) as isize);
    *v.offset((len - 1 as std::os::raw::c_int) as isize) = *v.offset(st as isize);
    *v.offset(st as isize) = tmp;
    return if k == st {
        *v.offset(st as isize)
    } else if st > k {
        qselect(v, st, k)
    } else {
        qselect(v.offset(st as isize), len - st, k - st)
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: [std::os::raw::c_int; 10] = [
        9 as std::os::raw::c_int,
        8 as std::os::raw::c_int,
        7 as std::os::raw::c_int,
        6 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
    ];
    let mut y: [std::os::raw::c_int; 10] = [0; 10];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        memcpy(
            y.as_mut_ptr() as *mut std::os::raw::c_void,
            x.as_mut_ptr() as *const std::os::raw::c_void,
            ::core::mem::size_of::<[std::os::raw::c_int; 10]>() as std::os::raw::c_ulong,
        );
        printf(
            b"%d: %d\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            qselect(y.as_mut_ptr(), 10 as std::os::raw::c_int, i),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

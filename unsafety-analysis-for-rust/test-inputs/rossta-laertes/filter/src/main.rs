#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
}
#[no_mangle]
pub extern "C" fn even_sel(mut x: std::os::raw::c_int) -> std::os::raw::c_int {
    return (x & 1 as std::os::raw::c_int == 0) as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn tri_sel(mut x: std::os::raw::c_int) -> std::os::raw::c_int {
    return x % 3 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn grep<'a1>(
    mut in_0: * mut std::os::raw::c_int,
    mut len: std::os::raw::c_int,
    mut outlen: Option<&'a1 mut std::os::raw::c_int>,
    mut sel: Option::<unsafe extern "C" fn(std::os::raw::c_int) -> std::os::raw::c_int>,
    mut inplace: std::os::raw::c_int,
) -> * mut std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut out: *mut std::os::raw::c_int = 0 as *mut std::os::raw::c_int;
    if inplace != 0 {
        out = in_0;
    } else {
        out = malloc(
            (std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
                .wrapping_mul(len as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
    }
    j = 0 as std::os::raw::c_int;
    i = j;
    while i < len {
        if sel.expect("non-null function pointer")(*in_0.offset(i as isize)) != 0 {
            let fresh0 = j;
            j = j + 1;
            *out.offset(fresh0 as isize) = *in_0.offset(i as isize);
        }
        i += 1;
        i;
    }
    if inplace == 0 && j < len {
        out = realloc(
            out as *mut u8,
            (std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
                .wrapping_mul(j as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_int;
    }
    *borrow_mut(&mut outlen).unwrap() = j;
    return out;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut in_0: [std::os::raw::c_int; 10] = [
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        6 as std::os::raw::c_int,
        7 as std::os::raw::c_int,
        8 as std::os::raw::c_int,
        9 as std::os::raw::c_int,
        10 as std::os::raw::c_int,
    ];
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut even: *mut std::os::raw::c_int = grep(
        in_0.as_mut_ptr(),
        10 as std::os::raw::c_int,
        Some(&mut len),
        Some(even_sel as unsafe extern "C" fn(std::os::raw::c_int) -> std::os::raw::c_int),
        0 as std::os::raw::c_int,
    );
    printf(b"Filtered even:\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < len {
        printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, *even.offset(i as isize));
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    grep(
        in_0.as_mut_ptr(),
        8 as std::os::raw::c_int,
        Some(&mut len),
        Some(tri_sel as unsafe extern "C" fn(std::os::raw::c_int) -> std::os::raw::c_int),
        1 as std::os::raw::c_int,
    );
    printf(
        b"In-place filtered not multiple of 3:\0" as *const u8 as *const std::os::raw::c_char,
    );
    i = 0 as std::os::raw::c_int;
    while i < len {
        printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, in_0[i as usize]);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

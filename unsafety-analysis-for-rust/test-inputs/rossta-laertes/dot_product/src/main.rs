#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut a: [std::os::raw::c_int; 3] = [
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        -(5 as std::os::raw::c_int),
    ];
    let mut b: [std::os::raw::c_int; 3] = [
        4 as std::os::raw::c_int,
        -(2 as std::os::raw::c_int),
        -(1 as std::os::raw::c_int),
    ];
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        dot_product(
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            (std::mem::size_of::<[std::os::raw::c_int; 3]>() as std::os::raw::c_ulong)
                .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
        ),
    );
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dot_product(
    mut a: *mut std::os::raw::c_int,
    mut b: *mut std::os::raw::c_int,
    mut n: size_t,
) -> std::os::raw::c_int {
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < n {
        sum += *a.offset(i as isize) * *b.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

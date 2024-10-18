#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type intFn = Option<unsafe extern "C"  fn(_: std::os::raw::c_int,_: std::os::raw::c_int,) -> std::os::raw::c_int>;
#[no_mangle]
pub unsafe extern "C" fn reduce(
    mut fn_0: intFn,
    mut size: std::os::raw::c_int,
    mut elms: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut val: std::os::raw::c_int = *elms;
    i = 1 as std::os::raw::c_int;
    while i < size {
        val = fn_0.expect("non-null function pointer")(val, *elms.offset(i as isize));
        i += 1;
        i;
    }
    return val;
}
#[no_mangle]
pub extern "C" fn add(mut a: std::os::raw::c_int, mut b: std::os::raw::c_int) -> std::os::raw::c_int {
    return a + b;
}
#[no_mangle]
pub extern "C" fn sub(mut a: std::os::raw::c_int, mut b: std::os::raw::c_int) -> std::os::raw::c_int {
    return a - b;
}
#[no_mangle]
pub extern "C" fn mul(mut a: std::os::raw::c_int, mut b: std::os::raw::c_int) -> std::os::raw::c_int {
    return a * b;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut nums: [std::os::raw::c_int; 5] = [
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
    ];
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        reduce(
            Some(add as unsafe extern "C" fn(std::os::raw::c_int, std::os::raw::c_int) -> std::os::raw::c_int),
            5 as std::os::raw::c_int,
            nums.as_mut_ptr(),
        ),
    );
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        reduce(
            Some(sub as unsafe extern "C" fn(std::os::raw::c_int, std::os::raw::c_int) -> std::os::raw::c_int),
            5 as std::os::raw::c_int,
            nums.as_mut_ptr(),
        ),
    );
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        reduce(
            Some(mul as unsafe extern "C" fn(std::os::raw::c_int, std::os::raw::c_int) -> std::os::raw::c_int),
            5 as std::os::raw::c_int,
            nums.as_mut_ptr(),
        ),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

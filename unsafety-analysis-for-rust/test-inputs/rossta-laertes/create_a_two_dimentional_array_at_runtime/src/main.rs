#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn scanf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut user1: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut user2: std::os::raw::c_int = 0 as std::os::raw::c_int;
    printf(
        b"Enter two integers.  Space delimited, please:  \0" as *const u8
            as *const std::os::raw::c_char,
    );
    scanf(
        b"%d %d\0" as *const u8 as *const std::os::raw::c_char,
        &mut user1 as *mut std::os::raw::c_int,
        &mut user2 as *mut std::os::raw::c_int,
    );
    let vla = user1 as usize;
    let vla_0 = user2 as usize;
    let mut array: Vec::<std::os::raw::c_int> = ::std::vec::from_elem(0, vla * vla_0);
    *array
        .as_mut_ptr()
        .offset((user1 / 2 as std::os::raw::c_int) as isize * vla_0 as isize)
        .offset((user2 / 2 as std::os::raw::c_int) as isize) = user1 + user2;
    printf(
        b"array[%d][%d] is %d\n\0" as *const u8 as *const std::os::raw::c_char,
        user1 / 2 as std::os::raw::c_int,
        user2 / 2 as std::os::raw::c_int,
        *array
            .as_mut_ptr()
            .offset((user1 / 2 as std::os::raw::c_int) as isize * vla_0 as isize)
            .offset((user2 / 2 as std::os::raw::c_int) as isize),
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

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe extern "C" fn callbackFunction(
    mut location: libc::c_int,
    mut value: libc::c_int,
) {
    printf(b"array[%d] = %d\n\0" as *const u8 as *const libc::c_char, location, value);
}
#[no_mangle]
pub unsafe extern "C" fn map(
    mut array: *mut libc::c_int,
    mut len: libc::c_int,
    mut callback: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        callback.expect("non-null function pointer")(i, *array.offset(i as isize));
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut array: [libc::c_int; 4] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
    ];
    map(
        array.as_mut_ptr(),
        4 as libc::c_int,
        Some(callbackFunction as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

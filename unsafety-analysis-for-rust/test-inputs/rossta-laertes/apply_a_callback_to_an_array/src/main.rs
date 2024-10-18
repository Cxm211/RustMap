#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe extern "C" fn callbackFunction(
    mut location: std::os::raw::c_int,
    mut value: std::os::raw::c_int,
) {
    printf(b"array[%d] = %d\n\0" as *const u8 as *const std::os::raw::c_char, location, value);
}
#[no_mangle]
pub unsafe extern "C" fn map(
    mut array: *mut std::os::raw::c_int,
    mut len: std::os::raw::c_int,
    mut callback: Option::<unsafe extern "C" fn(std::os::raw::c_int, std::os::raw::c_int) -> ()>,
) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < len {
        callback.expect("non-null function pointer")(i, *array.offset(i as isize));
        i += 1;
        i;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut array: [std::os::raw::c_int; 4] = [
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
    ];
    map(
        array.as_mut_ptr(),
        4 as std::os::raw::c_int,
        Some(callbackFunction as unsafe extern "C" fn(std::os::raw::c_int, std::os::raw::c_int) -> ()),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

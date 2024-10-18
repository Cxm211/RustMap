#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
#[no_mangle]
pub extern "C" fn number_of_digits(mut x: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut NumberOfDigits: std::os::raw::c_int = 0;
    NumberOfDigits = 0 as std::os::raw::c_int;
    while x != 0 as std::os::raw::c_int {
        x = x / 10 as std::os::raw::c_int;
        NumberOfDigits += 1;
        NumberOfDigits;
    }
    return NumberOfDigits;
}
#[no_mangle]
pub unsafe extern "C" fn convert_array(
    mut array: *mut std::os::raw::c_char,
    mut NumberOfElements: std::os::raw::c_int,
) -> *mut std::os::raw::c_int {
    let mut convertedArray: *mut std::os::raw::c_int = malloc(
        (NumberOfElements as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut originalElement: std::os::raw::c_int = 0;
    let mut convertedElement: std::os::raw::c_int = 0;
    convertedElement = 0 as std::os::raw::c_int;
    originalElement = 0 as std::os::raw::c_int;
    while convertedElement < NumberOfElements {
        *convertedArray
            .offset(
                convertedElement as isize,
            ) = atoi(&mut *array.offset(originalElement as isize));
        originalElement
            += number_of_digits(*convertedArray.offset(convertedElement as isize))
                + 1 as std::os::raw::c_int;
        convertedElement += 1;
        convertedElement;
    }
    return convertedArray;
}
#[no_mangle]
pub unsafe extern "C" fn isSorted(
    mut array: *mut std::os::raw::c_int,
    mut numberOfElements: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut sorted: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut counter: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while counter < numberOfElements {
        if counter != 0 as std::os::raw::c_int
            && *array.offset((counter - 1 as std::os::raw::c_int) as isize)
                > *array.offset(counter as isize)
        {
            sorted -= 1;
            sorted;
        }
        counter += 1;
        counter;
    }
    return sorted;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut convertedArray: *mut std::os::raw::c_int = core::ptr::null_mut();
    convertedArray = convert_array(
        *argv.offset(1 as std::os::raw::c_int as isize),
        argc - 1 as std::os::raw::c_int,
    );
    if isSorted(convertedArray, argc - 1 as std::os::raw::c_int) == 1 as std::os::raw::c_int {
        printf(
            b"Did you forgot to turn on your brain?! This array is already sorted!\n\0"
                as *const u8 as *const std::os::raw::c_char,
        );
    } else if argc - 1 as std::os::raw::c_int <= 10 as std::os::raw::c_int {
        printf(
            b"Am I really supposed to sort this? Sort it by yourself!\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
    } else {
        printf(
            b"Am I really supposed to sort this? Bhahahaha!\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
    }
    free(convertedArray as *mut u8);
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

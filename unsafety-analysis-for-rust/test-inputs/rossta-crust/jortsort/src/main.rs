#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn number_of_digits(mut x: i32) -> i32 {
    let mut NumberOfDigits: i32 = 0;
    NumberOfDigits = 0;
    while x != 0 {
        x = x / 10;
        NumberOfDigits += 1;
        NumberOfDigits;
    }
    return NumberOfDigits;
}

#[no_mangle]
pub extern "C" fn convert_array(mut array: *mut i8, mut NumberOfElements: i32) -> *mut i32 {
    unsafe {
        let mut convertedArray: *mut i32 =
            malloc((NumberOfElements as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64))
                as *mut i32;
        let mut originalElement: i32 = 0;
        let mut convertedElement: i32 = 0;
        convertedElement = 0;
        originalElement = 0;
        while convertedElement < NumberOfElements {
            *convertedArray.offset(convertedElement as isize) =
                atoi(&mut *array.offset(originalElement as isize));
            originalElement +=
                number_of_digits(*convertedArray.offset(convertedElement as isize)) + 1;
            convertedElement += 1;
            convertedElement;
        }
        return convertedArray;
    }
}

#[no_mangle]
pub extern "C" fn isSorted(mut array: *mut i32, mut numberOfElements: i32) -> i32 {
    unsafe {
        let mut sorted: i32 = 1;
        let mut counter: i32 = 0;
        while counter < numberOfElements {
            if counter != 0
                && *array.offset((counter - 1i32) as isize) > *array.offset(counter as isize)
            {
                sorted -= 1;
                sorted;
            }
            counter += 1;
            counter;
        }
        return sorted;
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut convertedArray: *mut i32 = 0 as *mut i32;
        convertedArray = convert_array(*argv.offset(1 as isize), argc - 1);
        if isSorted(convertedArray, argc - 1) == 1 {
            print!("Did you forgot to turn on your brain?! This array is already sorted!\n");
        } else if argc - 1 <= 10 {
            print!("Am I really supposed to sort this? Sort it by yourself!\n");
        } else {
            print!("Am I really supposed to sort this? Bhahahaha!\n");
        }
        free(convertedArray as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}

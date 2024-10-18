#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn log2(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn print_headings() {
    printf(
        b"%2s\0" as *const u8 as *const std::os::raw::c_char,
        b"N\0" as *const u8 as *const std::os::raw::c_char,
    );
    printf(
        b" %10s\0" as *const u8 as *const std::os::raw::c_char,
        b"Length\0" as *const u8 as *const std::os::raw::c_char,
    );
    printf(
        b" %-20s\0" as *const u8 as *const std::os::raw::c_char,
        b"Entropy\0" as *const u8 as *const std::os::raw::c_char,
    );
    printf(
        b" %-40s\0" as *const u8 as *const std::os::raw::c_char,
        b"Word\0" as *const u8 as *const std::os::raw::c_char,
    );
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn calculate_entropy(
    mut ones: std::os::raw::c_int,
    mut zeros: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut result: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut total: std::os::raw::c_int = ones + zeros;
    result
        -= ones as std::os::raw::c_double / total as std::os::raw::c_double
            * log2(ones as std::os::raw::c_double / total as std::os::raw::c_double);
    result
        -= zeros as std::os::raw::c_double / total as std::os::raw::c_double
            * log2(zeros as std::os::raw::c_double / total as std::os::raw::c_double);
    if result != result {
        result = 0 as std::os::raw::c_int as std::os::raw::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn print_entropy(mut word: *mut std::os::raw::c_char) {
    let mut ones: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut zeros: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while *word.offset(i as isize) != 0 {
        let mut c: std::os::raw::c_char = *word.offset(i as isize);
        match c as std::os::raw::c_int {
            48 => {
                zeros += 1;
                zeros;
            }
            49 => {
                ones += 1;
                ones;
            }
            _ => {}
        }
        i += 1;
        i;
    }
    let mut entropy: std::os::raw::c_double = calculate_entropy(ones, zeros);
    printf(b" %-20.18f\0" as *const u8 as *const std::os::raw::c_char, entropy);
}
#[no_mangle]
pub unsafe extern "C" fn print_word(mut n: std::os::raw::c_int, mut word: *mut std::os::raw::c_char) {
    printf(b"%2d\0" as *const u8 as *const std::os::raw::c_char, n);
    printf(b" %10ld\0" as *const u8 as *const std::os::raw::c_char, strlen(word));
    print_entropy(word);
    if n < 10 as std::os::raw::c_int {
        printf(b" %-40s\0" as *const u8 as *const std::os::raw::c_char, word);
    } else {
        printf(
            b" %-40s\0" as *const u8 as *const std::os::raw::c_char,
            b"...\0" as *const u8 as *const std::os::raw::c_char,
        );
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    print_headings();
    let mut last_word: *mut std::os::raw::c_char = malloc(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        as *mut std::os::raw::c_char;
    strcpy(last_word, b"1\0" as *const u8 as *const std::os::raw::c_char);
    let mut current_word: *mut std::os::raw::c_char = malloc(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        as *mut std::os::raw::c_char;
    strcpy(current_word, b"0\0" as *const u8 as *const std::os::raw::c_char);
    print_word(1 as std::os::raw::c_int, last_word);
    let mut i: std::os::raw::c_int = 0;
    i = 2 as std::os::raw::c_int;
    while i <= 37 as std::os::raw::c_int {
        print_word(i, current_word);
        let mut next_word: *mut std::os::raw::c_char = malloc(
            (strlen(current_word))
                .wrapping_add(strlen(last_word))
                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
        ) as *mut std::os::raw::c_char;
        strcpy(next_word, current_word);
        strcat(next_word, last_word);
        free(last_word as *mut u8);
        last_word = current_word;
        current_word = next_word;
        i += 1;
        i;
    }
    free(last_word as *mut u8);
    free(current_word as *mut u8);
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

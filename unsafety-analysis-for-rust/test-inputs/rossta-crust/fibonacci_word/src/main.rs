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
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

//use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn log2(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn print_headings() {
    print!("{:2}", "N\0");
    print!(" {:10}", "Length\0");
    print!(" {:-20}", "Entropy\0");
    print!(" {:-40}", "Word\0");
    print!("\n");
}

#[no_mangle]
pub extern "C" fn calculate_entropy(mut ones: i32, mut zeros: i32) -> f64 {
    let mut result: f64 = 0 as f64;
    let mut total: i32 = ones + zeros;
    unsafe {
        result -= ones as f64 / total as f64 * log2(ones as f64 / total as f64);
        result -= zeros as f64 / total as f64 * log2(zeros as f64 / total as f64);
    }
    if result != result {
        result = 0 as f64;
    }
    return result;
}

#[no_mangle]
pub extern "C" fn print_entropy(mut word: *mut i8) {
    unsafe {
        let mut ones: i32 = 0;
        let mut zeros: i32 = 0;
        let mut i: i32 = 0;
        i = 0;
        while *word.offset(i as isize) != 0 {
            let mut c: i8 = *word.offset(i as isize);
            match c as i32 {
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
        let mut entropy: f64 = calculate_entropy(ones, zeros);
        print!(" {:-20.18}", entropy);
    }
}

#[no_mangle]
pub extern "C" fn print_word(mut n: i32, mut word: *mut i8) {
    unsafe {
        print!("{:2}", n);
        print!(" {:10}", strlen(word));
        print_entropy(word);
        if n < 10 {
            print!(" {:-40}", build_str_from_raw_ptr(word as *mut u8));
        } else {
            print!(" {:-40}", "...\0");
        }
        print!("\n");
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        print_headings();
        let mut last_word: *mut i8 = malloc(2) as *mut i8;
        strcpy(last_word, b"1\0" as *const u8 as *const i8);
        let mut current_word: *mut i8 = malloc(2) as *mut i8;
        strcpy(current_word, b"0\0" as *const u8 as *const i8);
        print_word(1, last_word);
        let mut i: i32 = 0;
        i = 2;
        while i <= 37 {
            print_word(i, current_word);
            let mut next_word: *mut i8 = malloc(
                (strlen(current_word))
                    .wrapping_add(strlen(last_word))
                    .wrapping_add(1),
            ) as *mut i8;
            strcpy(next_word, current_word);
            strcat(next_word, last_word);
            free(last_word as *mut libc::c_void);
            last_word = current_word;
            current_word = next_word;
            i += 1;
            i;
        }
        free(last_word as *mut libc::c_void);
        free(current_word as *mut libc::c_void);
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

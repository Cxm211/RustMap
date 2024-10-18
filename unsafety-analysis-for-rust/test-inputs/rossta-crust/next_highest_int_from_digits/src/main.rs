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
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn strtoul(_: *const i8, _: *mut *mut i8, _: i32) -> u64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn swap(mut str: *mut i8, mut i: i32, mut j: i32) {
    unsafe {
        let mut c: i8 = *str.offset(i as isize);
        *str.offset(i as isize) = *str.offset(j as isize);
        *str.offset(j as isize) = c;
    }
}

#[no_mangle]
pub extern "C" fn reverse(mut str: *mut i8, mut i: i32, mut j: i32) {
    unsafe {
        while i < j {
            swap(str, i, j);
            i += 1;
            i;
            j -= 1;
            j;
        }
    }
}

#[no_mangle]
pub extern "C" fn next_permutation(mut str: *mut i8) -> bool {
    unsafe {
        let mut len: i32 = strlen(str) as i32;
        if len < 2 {
            return 0 != 0;
        }
        let mut i: i32 = len - 1;
        while i > 0 {
            let mut j: i32 = i;
            let mut k: i32 = 0;
            i -= 1;
            if (*str.offset(i as isize) as i32) < *str.offset(j as isize) as i32 {
                k = len;
                loop {
                    k -= 1;
                    if !(*str.offset(i as isize) as i32 >= *str.offset(k as isize) as i32) {
                        break;
                    }
                }
                swap(str, i, k);
                reverse(str, j, len - 1);
                return 1 != 0;
            }
        }
        return 0 != 0;
    }
}

#[no_mangle]
pub extern "C" fn next_highest_int(mut n: u32) -> u32 {
    let mut str: [i8; 16] = [0; 16];
    unsafe {
        snprintf(
            str.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 16]>() as u64,
            b"%u\0" as *const u8 as *const i8,
            n,
        );
    }
    if !next_permutation(str.as_mut_ptr()) {
        return 0;
    }
    unsafe {
        return strtoul(str.as_mut_ptr(), 0 as *mut *mut i8, 10) as u32;
    }
}

fn main_0() -> i32 {
    let mut numbers: [u32; 8] = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    let count: i32 = (::core::mem::size_of::<[u32; 8]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    let mut i: i32 = 0;
    while i < count {
        print!(
            "{} -> {}\n",
            numbers[i as usize],
            next_highest_int(numbers[i as usize])
        );
        i += 1;
        i;
    }
    unsafe {
        let big: [i8; 23] =
            *::core::mem::transmute::<&[u8; 23], &[i8; 23]>(b"9589776899767587796600\0");
        let mut next: [i8; 23] = [0; 23];
        memcpy(
            next.as_mut_ptr() as *mut libc::c_void,
            big.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[i8; 23]>() as u64,
        );
        next_permutation(next.as_mut_ptr());
        print!(
            "{} -> {}\n",
            build_str_from_raw_ptr(big.as_ptr() as *mut u8),
            build_str_from_raw_ptr(next.as_mut_ptr() as *mut u8)
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

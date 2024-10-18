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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn incr(mut s: *mut i8) -> *mut i8 {
    unsafe {
        let mut i: i32 = 0;
        let mut begin: i32 = 0;
        let mut tail: i32 = 0;
        let mut len: i32 = 0;
        let mut neg: i32 = (*s as i32 == '-' as i32) as i32;
        let mut tgt: i8 = (if neg != 0 { '0' as i32 } else { '9' as i32 }) as i8;
        if strcmp(s, b"-1\0" as *const u8 as *const i8) == 0 {
            *s.offset(0 as isize) = '0' as i8;
            *s.offset(1 as isize) = '\0' as i8;
            return s;
        }
        len = strlen(s) as i32;
        begin = if *s as i32 == '-' as i32 || *s as i32 == '+' as i32 {
            1
        } else {
            0
        };
        tail = len - 1;
        while tail >= begin && *s.offset(tail as isize) as i32 == tgt as i32 {
            tail -= 1;
            tail;
        }
        if tail < begin && neg == 0 {
            if begin == 0 {
                s = realloc(s as *mut libc::c_void, (len + 2i32) as u64) as *mut i8;
            };
            *s.offset(0 as isize) = '1' as i8;
            i = 1;
            while i <= len - begin {
                *s.offset(i as isize) = '0' as i8;
                i += 1;
                i;
            }
            *s.offset((len + 1i32) as isize) = '\0' as i8;
        } else if tail == begin && neg != 0 && *s.offset(1 as isize) as i32 == '1' as i32 {
            i = 1;
            while i < len - begin {
                *s.offset(i as isize) = '9' as i8;
                i += 1;
                i;
            }
            *s.offset((len - 1i32) as isize) = '\0' as i8;
        } else {
            i = len - 1;
            while i > tail {
                *s.offset(i as isize) = (if neg != 0 { '9' as i32 } else { '0' as i32 }) as i8;
                i -= 1;
                i;
            }
            let ref mut fresh0 = *s.offset(tail as isize);
            *fresh0 = (*fresh0 as i32 + if neg != 0 { -1i32 } else { 1 }) as i8;
        }
        return s;
    }
}

#[no_mangle]
pub extern "C" fn string_test(mut s: *const i8) {
    unsafe {
        let mut ret: *mut i8 = malloc(strlen(s)) as *mut i8;
        strcpy(ret, s);
        print!("text: {}\n", build_str_from_raw_ptr(ret as *mut u8));
        ret = incr(ret);
        print!("  ->: {}\n", build_str_from_raw_ptr(ret as *mut u8));
        free(ret as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    string_test(b"+0\0" as *const u8 as *const i8);
    string_test(b"-1\0" as *const u8 as *const i8);
    string_test(b"-41\0" as *const u8 as *const i8);
    string_test(b"+41\0" as *const u8 as *const i8);
    string_test(b"999\0" as *const u8 as *const i8);
    string_test(b"+999\0" as *const u8 as *const i8);
    string_test(b"109999999999999999999999999999999999999999\0" as *const u8 as *const i8);
    string_test(b"-100000000000000000000000000000000000000000000\0" as *const u8 as *const i8);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

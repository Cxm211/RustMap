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
    fn perror(__s: *const i8);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
}
#[no_mangle]
pub extern "C" fn quib(mut strs: *mut *const i8, mut size: u64) -> *mut i8 {
    unsafe {
        let mut len: u64 = 3u64.wrapping_add(
            (if size > 1 {
                2u64.wrapping_mul(size).wrapping_add(1)
            } else {
                0
            }),
        );
        let mut i: u64 = 0;
        i = 0;
        while i < size {
            len = (len).wrapping_add(strlen(*strs.offset(i as isize))) as u64;
            i = i.wrapping_add(1);
            i;
        }
        let mut s: *mut i8 =
            malloc(len.wrapping_mul(::core::mem::size_of::<i8>() as u64)) as *mut i8;
        if s.is_null() {
            perror(b"Can't allocate memory!\n\0" as *const u8 as *const i8);
            exit(1);
        }
        strcpy(s, b"{\0" as *const u8 as *const i8);
        match size {
            0 => {}
            1 => {
                strcat(s, *strs.offset(0 as isize));
            }
            _ => {
                i = 0;
                while i < size.wrapping_sub(1) {
                    strcat(s, *strs.offset(i as isize));
                    if i < size.wrapping_sub(2) {
                        strcat(s, b", \0" as *const u8 as *const i8);
                    } else {
                        strcat(s, b" and \0" as *const u8 as *const i8);
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                strcat(s, *strs.offset(i as isize));
            }
        }
        strcat(s, b"}\0" as *const u8 as *const i8);
        return s;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut test: [*const i8; 4] = [
            b"ABC\0" as *const u8 as *const i8,
            b"DEF\0" as *const u8 as *const i8,
            b"G\0" as *const u8 as *const i8,
            b"H\0" as *const u8 as *const i8,
        ];
        let mut s: *mut i8 = 0 as *mut i8;
        let mut i: u64 = 0;
        while i < 5 {
            s = quib(test.as_mut_ptr(), i);
            print!("{}\n", build_str_from_raw_ptr(s as *mut u8));
            free(s as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        return 0;
    }
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}

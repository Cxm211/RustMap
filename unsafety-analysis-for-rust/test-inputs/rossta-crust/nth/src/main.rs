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
}
#[no_mangle]
pub extern "C" fn addSuffix(mut num: i32, mut buf: *mut i8, mut len: u64) -> *mut i8 {
    unsafe {
        let mut suffixes: [*mut i8; 4] = [
            b"th\0" as *const u8 as *const i8 as *mut i8,
            b"st\0" as *const u8 as *const i8 as *mut i8,
            b"nd\0" as *const u8 as *const i8 as *mut i8,
            b"rd\0" as *const u8 as *const i8 as *mut i8,
        ];
        let mut i: i32 = 0;
        match num % 10 {
            1 => {
                i = if num % 100 == 11 { 0 } else { 1 };
            }
            2 => {
                i = if num % 100 == 12 { 0 } else { 2 };
            }
            3 => {
                i = if num % 100 == 13 { 0 } else { 3 };
            }
            _ => {
                i = 0;
            }
        }
        snprintf(
            buf,
            len,
            b"%d%s\0" as *const u8 as *const i8,
            num,
            suffixes[i as usize],
        );
        return buf;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    print!("Set [0,25]:\n");
    i = 0;
    unsafe {
        while i < 26 {
            let mut s: [i8; 5] = [0; 5];
            print!(
                "{} ",
                build_str_from_raw_ptr(addSuffix(i, s.as_mut_ptr(), 5) as *mut u8)
            );
            i += 1;
            i;
        }
    }
    print!("{}", '\n' as i32);
    print!("Set [250,265]:\n");
    i = 250;
    unsafe {
        while i < 266 {
            let mut s_0: [i8; 6] = [0; 6];
            print!(
                "{} ",
                build_str_from_raw_ptr(addSuffix(i, s_0.as_mut_ptr(), 6) as *mut u8)
            );
            i += 1;
            i;
        }
    }
    print!("{}", '\n' as i32);
    print!("Set [1000,1025]:\n");
    i = 1000;
    unsafe {
        while i < 1026 {
            let mut s_1: [i8; 7] = [0; 7];
            print!(
                "{} ",
                build_str_from_raw_ptr(addSuffix(i, s_1.as_mut_ptr(), 7) as *mut u8)
            );
            i += 1;
            i;
        }
    }
    print!("{}", '\n' as i32);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

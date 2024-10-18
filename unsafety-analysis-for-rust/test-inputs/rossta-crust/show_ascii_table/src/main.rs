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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: [i8; 4] = [0; 4];
    i = 0;
    unsafe {
        while i < 16 {
            j = 32 + i;
            while j < 128 {
                match j {
                    32 => {
                        sprintf(k.as_mut_ptr(), b"Spc\0" as *const u8 as *const i8);
                    }
                    127 => {
                        sprintf(k.as_mut_ptr(), b"Del\0" as *const u8 as *const i8);
                    }
                    _ => {
                        sprintf(k.as_mut_ptr(), b"%c\0" as *const u8 as *const i8, j);
                    }
                }
                print!(
                    "{:3} : {:-3}   ",
                    j,
                    build_str_from_raw_ptr(k.as_mut_ptr() as *mut u8)
                );
                j += 16;
            }
            print!("\n");
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

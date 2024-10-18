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
    fn puts(__s: *const i8) -> i32;
}
#[no_mangle]
pub extern "C" fn damm(mut input: *mut u8, mut length: u64) -> bool {
    unsafe {
        static mut table: [[u8; 10]; 10] = [
            [0, 3, 1, 7, 5, 9, 8, 6, 4, 2],
            [7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
            [4, 2, 0, 6, 8, 7, 1, 3, 5, 9],
            [1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
            [6, 1, 2, 3, 0, 4, 5, 9, 7, 8],
            [3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
            [5, 8, 6, 9, 7, 2, 0, 1, 3, 4],
            [8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
            [9, 4, 3, 8, 6, 1, 7, 2, 0, 5],
            [2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
        ];
        let mut interim: u8 = 0;
        let mut i: u64 = 0;
        while i < length {
            interim = table[interim as usize][*input.offset(i as isize) as usize];
            i = i.wrapping_add(1);
            i;
        }
        return interim as i32 == 0;
    }
}

fn main_0() -> i32 {
    let mut input: [u8; 4] = [5, 7, 2, 4];
    unsafe {
        puts(if damm(input.as_mut_ptr(), 4) as i32 != 0 {
            b"Checksum correct\0" as *const u8 as *const i8
        } else {
            b"Checksum incorrect\0" as *const u8 as *const i8
        });
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

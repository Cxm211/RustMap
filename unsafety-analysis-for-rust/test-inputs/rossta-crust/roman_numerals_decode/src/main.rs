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
extern "C" {}
#[no_mangle]
pub static mut digits: [i32; 26] = [
    0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0,
];
#[no_mangle]
pub extern "C" fn decode(mut roman: *const i8) -> i32 {
    unsafe {
        let mut bigger: *const i8 = 0 as *const i8;
        let mut current: i32 = 0;
        let mut arabic: i32 = 0;
        while *roman as i32 != '\0' as i32 {
            current = digits[((!0x20i32 & *roman as i32) - 'A' as i32) as usize];
            bigger = roman;
            while digits[((!0x20i32 & *bigger as i32) - 'A' as i32) as usize] <= current && {
                bigger = bigger.offset(1);
                *bigger as i32 != '\0' as i32
            } {}
            if *bigger as i32 == '\0' as i32 {
                arabic += current;
            } else {
                arabic += digits[((!0x20i32 & *bigger as i32) - 'A' as i32) as usize];
                while roman < bigger {
                    let fresh0 = roman;
                    roman = roman.offset(1);
                    arabic -= digits[((!0x20i32 & *fresh0 as i32) - 'A' as i32) as usize];
                }
            }
            roman = roman.offset(1);
            roman;
        }
        return arabic;
    }
}

fn main_0() -> i32 {
    let mut romans: [*const i8; 4] = [
        b"MCmxC\0" as *const u8 as *const i8,
        b"MMVIII\0" as *const u8 as *const i8,
        b"MDClXVI\0" as *const u8 as *const i8,
        b"MCXLUJ\0" as *const u8 as *const i8,
    ];
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while i < 4 {
            print!(
                "{}	{}\n",
                build_str_from_raw_ptr(romans[i as usize] as *mut u8),
                decode(romans[i as usize])
            );
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

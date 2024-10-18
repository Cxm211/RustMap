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
extern "C" {}
#[no_mangle]
pub extern "C" fn is_leap_year(mut year: i32) -> i32 {
    return if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        1
    } else {
        0
    };
}

fn main_0() -> i32 {
    let mut test_case: [i32; 5] = [1900, 1994, 1996, 1997, 2000];
    let mut key: i32 = 0;
    let mut end: i32 = 0;
    let mut year: i32 = 0;
    key = 0;
    end = (::core::mem::size_of::<[i32; 5]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    while key < end {
        year = test_case[key as usize];
        if is_leap_year(year) == 1 {
            print!("{} is {}a leap year.\n", year, "\0")
        } else {
            print!("{} is {}a leap year.\n", year, "not \0")
        };
        key += 1;
        key;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

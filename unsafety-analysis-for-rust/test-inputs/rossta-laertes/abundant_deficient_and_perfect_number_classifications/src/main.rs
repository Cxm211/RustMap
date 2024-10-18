#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut try_max: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut count_list: [std::os::raw::c_int; 3] = [
        1 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ];
    i = 2 as std::os::raw::c_int;
    while i <= 20000 as std::os::raw::c_int {
        try_max = i / 2 as std::os::raw::c_int;
        sum = 1 as std::os::raw::c_int;
        j = 2 as std::os::raw::c_int;
        while j < try_max {
            if !(i % j != 0) {
                try_max = i / j;
                sum += j;
                if j != try_max {
                    sum += try_max;
                }
            }
            j += 1;
            j;
        }
        if sum < i {
            count_list[0 as std::os::raw::c_int as usize] += 1;
            count_list[0 as std::os::raw::c_int as usize];
        } else if sum > i {
            count_list[2 as std::os::raw::c_int as usize] += 1;
            count_list[2 as std::os::raw::c_int as usize];
        } else {
            count_list[1 as std::os::raw::c_int as usize] += 1;
            count_list[1 as std::os::raw::c_int as usize];
        }
        i += 1;
        i;
    }
    printf(
        b"\nThere are %d deficient,\0" as *const u8 as *const std::os::raw::c_char,
        count_list[0 as std::os::raw::c_int as usize],
    );
    printf(
        b" %d perfect,\0" as *const u8 as *const std::os::raw::c_char,
        count_list[1 as std::os::raw::c_int as usize],
    );
    printf(
        b" %d abundant numbers between 1 and 20000.\n\0" as *const u8
            as *const std::os::raw::c_char,
        count_list[2 as std::os::raw::c_int as usize],
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

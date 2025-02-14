#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: std::os::raw::c_ulonglong = 1 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < 30 as std::os::raw::c_int {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, n.count_ones() as i32);
        n = n.wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulonglong);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    let mut od: [std::os::raw::c_int; 30] = [0; 30];
    let mut ne: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut no: std::os::raw::c_int = 0 as std::os::raw::c_int;
    printf(b"evil  : \0" as *const u8 as *const std::os::raw::c_char);
    let mut n_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while ne + no < 60 as std::os::raw::c_int {
        if (n_0 as std::os::raw::c_uint).count_ones() as i32 & 1 as std::os::raw::c_int
            == 0 as std::os::raw::c_int
        {
            if ne < 30 as std::os::raw::c_int {
                printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, n_0);
                ne += 1;
                ne;
            }
        } else if no < 30 as std::os::raw::c_int {
            let fresh0 = no;
            no = no + 1;
            od[fresh0 as usize] = n_0;
        }
        n_0 += 1;
        n_0;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    printf(b"odious: \0" as *const u8 as *const std::os::raw::c_char);
    let mut i_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i_0 < 30 as std::os::raw::c_int {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, od[i_0 as usize]);
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

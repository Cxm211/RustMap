#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luhn(mut cc: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let m: [std::os::raw::c_int; 10] = [
        0 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        6 as std::os::raw::c_int,
        8 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        7 as std::os::raw::c_int,
        9 as std::os::raw::c_int,
    ];
    let mut i: std::os::raw::c_int = 0;
    let mut odd: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = strlen(cc) as std::os::raw::c_int;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut digit: std::os::raw::c_int = *cc.offset(i as isize) as std::os::raw::c_int - '0' as i32;
        sum += if odd != 0 { digit } else { m[digit as usize] };
        odd = (odd == 0) as std::os::raw::c_int;
    }
    return (sum % 10 as std::os::raw::c_int == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut cc: [*const std::os::raw::c_char; 5] = [
        b"49927398716\0" as *const u8 as *const std::os::raw::c_char,
        b"49927398717\0" as *const u8 as *const std::os::raw::c_char,
        b"1234567812345678\0" as *const u8 as *const std::os::raw::c_char,
        b"1234567812345670\0" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_char,
    ];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while !(cc[i as usize]).is_null() {
        printf(
            b"%16s\t%s\n\0" as *const u8 as *const std::os::raw::c_char,
            cc[i as usize],
            if luhn(cc[i as usize]) != 0 {
                b"ok\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"not ok\0" as *const u8 as *const std::os::raw::c_char
            },
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

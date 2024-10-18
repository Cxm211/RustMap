#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub static mut digits: [std::os::raw::c_int; 26] = [
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    100 as std::os::raw::c_int,
    500 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    1 as std::os::raw::c_int,
    1 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    50 as std::os::raw::c_int,
    1000 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    5 as std::os::raw::c_int,
    5 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    10 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
    0 as std::os::raw::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn decode(mut roman: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut bigger: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut current: std::os::raw::c_int = 0;
    let mut arabic: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while *roman as std::os::raw::c_int != '\0' as i32 {
        current = digits[((!(0x20 as std::os::raw::c_int) & *roman as std::os::raw::c_int) - 'A' as i32)
            as usize];
        bigger = roman;
        while digits[((!(0x20 as std::os::raw::c_int) & *bigger as std::os::raw::c_int) - 'A' as i32)
            as usize] <= current
            && {
                bigger = bigger.offset(1);
                *bigger as std::os::raw::c_int != '\0' as i32
            }
        {}
        if *bigger as std::os::raw::c_int == '\0' as i32 {
            arabic += current;
        } else {
            arabic
                += digits[((!(0x20 as std::os::raw::c_int) & *bigger as std::os::raw::c_int)
                    - 'A' as i32) as usize];
            while roman < bigger {
                let fresh0 = roman;
                roman = roman.offset(1);
                arabic
                    -= digits[((!(0x20 as std::os::raw::c_int) & *fresh0 as std::os::raw::c_int)
                        - 'A' as i32) as usize];
            }
        }
        roman = roman.offset(1);
        roman;
    }
    return arabic;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut romans: [*const std::os::raw::c_char; 4] = [
        b"MCmxC\0" as *const u8 as *const std::os::raw::c_char,
        b"MMVIII\0" as *const u8 as *const std::os::raw::c_char,
        b"MDClXVI\0" as *const u8 as *const std::os::raw::c_char,
        b"MCXLUJ\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        printf(
            b"%s\t%d\n\0" as *const u8 as *const std::os::raw::c_char,
            romans[i as usize],
            decode(romans[i as usize]),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

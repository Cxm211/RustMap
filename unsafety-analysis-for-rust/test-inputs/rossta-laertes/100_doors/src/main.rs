#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut is_open: [std::os::raw::c_char; 100] = [
        0 as std::os::raw::c_int as std::os::raw::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut pass: std::os::raw::c_int = 0;
    let mut door: std::os::raw::c_int = 0;
    pass = 0 as std::os::raw::c_int;
    while pass < 100 as std::os::raw::c_int {
        door = pass;
        while door < 100 as std::os::raw::c_int {
            is_open[door
                as usize] = (is_open[door as usize] == 0) as std::os::raw::c_int as std::os::raw::c_char;
            door += pass + 1 as std::os::raw::c_int;
        }
        pass += 1;
        pass;
    }
    door = 0 as std::os::raw::c_int;
    while door < 100 as std::os::raw::c_int {
        printf(
            b"door #%d is %s.\n\0" as *const u8 as *const std::os::raw::c_char,
            door + 1 as std::os::raw::c_int,
            if is_open[door as usize] as std::os::raw::c_int != 0 {
                b"open\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"closed\0" as *const u8 as *const std::os::raw::c_char
            },
        );
        door += 1;
        door;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

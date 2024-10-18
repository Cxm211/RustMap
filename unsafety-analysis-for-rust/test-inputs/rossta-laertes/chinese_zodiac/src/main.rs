#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn floor(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub static mut animals: [*const std::os::raw::c_char; 12] = [
    b"Rat\0" as *const u8 as *const std::os::raw::c_char,
    b"Ox\0" as *const u8 as *const std::os::raw::c_char,
    b"Tiger\0" as *const u8 as *const std::os::raw::c_char,
    b"Rabbit\0" as *const u8 as *const std::os::raw::c_char,
    b"Dragon\0" as *const u8 as *const std::os::raw::c_char,
    b"Snake\0" as *const u8 as *const std::os::raw::c_char,
    b"Horse\0" as *const u8 as *const std::os::raw::c_char,
    b"Goat\0" as *const u8 as *const std::os::raw::c_char,
    b"Monkey\0" as *const u8 as *const std::os::raw::c_char,
    b"Rooster\0" as *const u8 as *const std::os::raw::c_char,
    b"Dog\0" as *const u8 as *const std::os::raw::c_char,
    b"Pig\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub static mut elements: [*const std::os::raw::c_char; 5] = [
    b"Wood\0" as *const u8 as *const std::os::raw::c_char,
    b"Fire\0" as *const u8 as *const std::os::raw::c_char,
    b"Earth\0" as *const u8 as *const std::os::raw::c_char,
    b"Metal\0" as *const u8 as *const std::os::raw::c_char,
    b"Water\0" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn getElement(mut year: std::os::raw::c_int) -> *const std::os::raw::c_char {
    let mut element: std::os::raw::c_int = floor(
        ((year - 4 as std::os::raw::c_int) % 10 as std::os::raw::c_int / 2 as std::os::raw::c_int)
            as std::os::raw::c_double,
    ) as std::os::raw::c_int;
    return elements[element as usize];
}
#[no_mangle]
pub unsafe extern "C" fn getAnimal(mut year: std::os::raw::c_int) -> *const std::os::raw::c_char {
    return animals[((year - 4 as std::os::raw::c_int) % 12 as std::os::raw::c_int) as usize];
}
#[no_mangle]
pub extern "C" fn getYY(mut year: std::os::raw::c_int) -> *const std::os::raw::c_char {
    if year % 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return b"yang\0" as *const u8 as *const std::os::raw::c_char
    } else {
        return b"yin\0" as *const u8 as *const std::os::raw::c_char
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut years: [std::os::raw::c_int; 6] = [
        1935 as std::os::raw::c_int,
        1938 as std::os::raw::c_int,
        1968 as std::os::raw::c_int,
        1972 as std::os::raw::c_int,
        1976 as std::os::raw::c_int,
        2017 as std::os::raw::c_int,
    ];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 6 as std::os::raw::c_int {
        let mut year: std::os::raw::c_int = years[i as usize];
        printf(
            b"%d is the year of the %s %s (%s).\n\0" as *const u8 as *const std::os::raw::c_char,
            year,
            getElement(year),
            getAnimal(year),
            getYY(year),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

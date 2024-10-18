#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn toupper(_: std::os::raw::c_int) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn can_make_words(
    mut b: *mut *mut std::os::raw::c_char,
    mut word: *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut c: std::os::raw::c_int = toupper(*word as std::os::raw::c_int);
    if c == 0 {
        return 1 as std::os::raw::c_int;
    }
    if (*b.offset(0 as std::os::raw::c_int as isize)).is_null() {
        return 0 as std::os::raw::c_int;
    }
    i = 0 as std::os::raw::c_int;
    while !(*b.offset(i as isize)).is_null() && ret == 0 {
        if !(*(*b.offset(i as isize)).offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            != c
            && *(*b.offset(i as isize)).offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                != c)
        {
            if *b.offset(i as isize) != *b.offset(0 as std::os::raw::c_int as isize) {
                let mut tmp: *mut std::os::raw::c_char = *b.offset(i as isize);
                let ref mut fresh0 = *b.offset(i as isize);
                *fresh0 = *b.offset(0 as std::os::raw::c_int as isize);
                let ref mut fresh1 = *b.offset(0 as std::os::raw::c_int as isize);
                *fresh1 = tmp;
            }
            ret = can_make_words(
                b.offset(1 as std::os::raw::c_int as isize),
                word.offset(1 as std::os::raw::c_int as isize),
            );
            if *b.offset(i as isize) != *b.offset(0 as std::os::raw::c_int as isize) {
                let mut tmp_0: *mut std::os::raw::c_char = *b.offset(i as isize);
                let ref mut fresh2 = *b.offset(i as isize);
                *fresh2 = *b.offset(0 as std::os::raw::c_int as isize);
                let ref mut fresh3 = *b.offset(0 as std::os::raw::c_int as isize);
                *fresh3 = tmp_0;
            }
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut blocks: [*mut std::os::raw::c_char; 21] = [
        b"BO\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"XK\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"DQ\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"CP\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"NA\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"GT\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"RE\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"TG\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"QD\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"FS\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"JW\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"HU\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"VI\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"AN\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"OB\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"ER\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"FS\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"LY\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"PC\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"ZM\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        core::ptr::null_mut(),
    ];
    let mut words: [*mut std::os::raw::c_char; 9] = [
        b"\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"A\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"BARK\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"BOOK\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"TREAT\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"COMMON\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"SQUAD\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"Confuse\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        core::ptr::null_mut(),
    ];
    let mut w: *mut *mut std::os::raw::c_char = core::ptr::null_mut();
    w = words.as_mut_ptr();
    while !(*w).is_null() {
        printf(
            b"%s\t%d\n\0" as *const u8 as *const std::os::raw::c_char,
            *w,
            can_make_words(blocks.as_mut_ptr(), *w),
        );
        w = w.offset(1);
        w;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

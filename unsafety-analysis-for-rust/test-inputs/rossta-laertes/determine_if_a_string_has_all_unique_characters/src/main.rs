#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct positionList {
    pub position: std::os::raw::c_int,
    pub next: * mut crate::positionList,
}
impl std::default::Default for positionList {
    fn default() -> Self {
        positionList {
        position: std::os::raw::c_int::default(),
        next: core::ptr::null_mut()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct letterList {
    pub letter: std::os::raw::c_char,
    pub repititions: std::os::raw::c_int,
    pub positions: * mut crate::positionList,
    pub next: * mut crate::letterList,
}
impl std::default::Default for letterList {
    fn default() -> Self {
        letterList {
        letter: std::os::raw::c_char::default(),
        repititions: std::os::raw::c_int::default(),
        positions: core::ptr::null_mut(),
        next: core::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub static mut letterSet: *mut letterList = 0 as *const letterList as *mut letterList;
#[no_mangle]
pub static mut duplicatesFound: bool = 0 as std::os::raw::c_int != 0;
#[no_mangle]
pub unsafe extern "C" fn checkAndUpdateLetterList(
    mut c: std::os::raw::c_char,
    mut pos: std::os::raw::c_int,
) {
    let mut letterOccurs: bool = 0 as std::os::raw::c_int != 0;
    let mut letterIterator: *mut letterList = core::ptr::null_mut();
    let mut newLetter: *mut letterList = core::ptr::null_mut();
    let mut positionIterator: *mut positionList = core::ptr::null_mut();
    let mut newPosition: *mut positionList = core::ptr::null_mut();
    if letterSet.is_null() {
        letterSet = malloc(std::mem::size_of::<letterList>() as std::os::raw::c_ulong)
            as *mut letterList;
        (*letterSet).letter = c;
        (*letterSet).repititions = 0 as std::os::raw::c_int;
        (*letterSet)
            .positions = malloc(std::mem::size_of::<positionList>() as std::os::raw::c_ulong)
            as *mut positionList;
        (*(*letterSet).positions).position = pos;
        (*(*letterSet).positions).next = core::ptr::null_mut();
        (*letterSet).next = core::ptr::null_mut();
    } else {
        letterIterator = letterSet;
        while !letterIterator.is_null() {
            if (*letterIterator).letter as std::os::raw::c_int == c as std::os::raw::c_int {
                letterOccurs = 1 as std::os::raw::c_int != 0;
                duplicatesFound = 1 as std::os::raw::c_int != 0;
                (*letterIterator).repititions += 1;
                (*letterIterator).repititions;
                positionIterator = (*letterIterator).positions;
                while !((*positionIterator).next).is_null() {
                    positionIterator = (*positionIterator).next;
                }
                newPosition = malloc(
                    std::mem::size_of::<positionList>() as std::os::raw::c_ulong,
                ) as *mut positionList;
                (*newPosition).position = pos;
                (*newPosition).next = core::ptr::null_mut();
                (*positionIterator).next = newPosition;
            }
            if letterOccurs as std::os::raw::c_int == 0 as std::os::raw::c_int
                && ((*letterIterator).next).is_null()
            {
                break;
            }
            letterIterator = (*letterIterator).next;
        }
        if letterOccurs as std::os::raw::c_int == 0 as std::os::raw::c_int {
            newLetter = malloc(std::mem::size_of::<letterList>() as std::os::raw::c_ulong)
                as *mut letterList;
            (*newLetter).letter = c;
            (*newLetter).repititions = 0 as std::os::raw::c_int;
            (*newLetter)
                .positions = malloc(
                std::mem::size_of::<positionList>() as std::os::raw::c_ulong,
            ) as *mut positionList;
            (*(*newLetter).positions).position = pos;
            (*(*newLetter).positions).next = core::ptr::null_mut();
            (*newLetter).next = core::ptr::null_mut();
            (*letterIterator).next = newLetter;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn printLetterList() {
    let mut positionIterator: *mut positionList = core::ptr::null_mut();
    let mut letterIterator: *mut letterList = letterSet;
    while !letterIterator.is_null() {
        if (*letterIterator).repititions > 0 as std::os::raw::c_int {
            printf(
                b"\n'%c' (0x%x) at positions :\0" as *const u8 as *const std::os::raw::c_char,
                (*letterIterator).letter as std::os::raw::c_int,
                (*letterIterator).letter as std::os::raw::c_int,
            );
            positionIterator = (*letterIterator).positions;
            while !positionIterator.is_null() {
                printf(
                    b"%3d\0" as *const u8 as *const std::os::raw::c_char,
                    (*positionIterator).position + 1 as std::os::raw::c_int,
                );
                positionIterator = (*positionIterator).next;
            }
        }
        letterIterator = (*letterIterator).next;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    if argc > 2 as std::os::raw::c_int {
        printf(
            b"Usage : %s <Test string>\n\0" as *const u8 as *const std::os::raw::c_char,
            *argv.offset(0 as std::os::raw::c_int as isize),
        );
        return 0 as std::os::raw::c_int;
    }
    if argc == 1 as std::os::raw::c_int
        || strlen(*argv.offset(1 as std::os::raw::c_int as isize))
            == 1 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        printf(
            b"\"%s\" - Length %d - Contains only unique characters.\n\0" as *const u8
                as *const std::os::raw::c_char,
            if argc == 1 as std::os::raw::c_int {
                b"\0" as *const u8 as *const std::os::raw::c_char
            } else {
                *argv.offset(1 as std::os::raw::c_int as isize) as *const std::os::raw::c_char
            },
            if argc == 1 as std::os::raw::c_int { 0 as std::os::raw::c_int } else { 1 as std::os::raw::c_int },
        );
        return 0 as std::os::raw::c_int;
    }
    len = strlen(*argv.offset(1 as std::os::raw::c_int as isize)) as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < len {
        checkAndUpdateLetterList(
            *(*argv.offset(1 as std::os::raw::c_int as isize)).offset(i as isize),
            i,
        );
        i += 1;
        i;
    }
    printf(
        b"\"%s\" - Length %d - %s\0" as *const u8 as *const std::os::raw::c_char,
        *argv.offset(1 as std::os::raw::c_int as isize),
        len,
        if duplicatesFound as std::os::raw::c_int == 0 as std::os::raw::c_int {
            b"Contains only unique characters.\n\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"Contains the following duplicate characters :\0" as *const u8
                as *const std::os::raw::c_char
        },
    );
    if duplicatesFound as std::os::raw::c_int == 1 as std::os::raw::c_int {
        printLetterList();
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

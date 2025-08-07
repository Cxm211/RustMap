#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn perror(__s: *const std::os::raw::c_char);
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
    fn exit(_: std::os::raw::c_int) -> !;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn quib(
    mut strs: *mut *const std::os::raw::c_char,
    mut size: size_t,
) -> *mut std::os::raw::c_char {
    let mut len: size_t = (3 as std::os::raw::c_int as std::os::raw::c_ulong)
        .wrapping_add(
            (if size > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                (2 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_mul(size)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_ulong
            }),
        );
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < size {
        len = (len as std::os::raw::c_ulong).wrapping_add(strlen(*strs.offset(i as isize)))
            as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    let mut s: *mut std::os::raw::c_char = malloc(
        len.wrapping_mul(::core::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    if s.is_null() {
        perror(b"Can't allocate memory!\n\0" as *const u8 as *const std::os::raw::c_char);
        exit(1 as std::os::raw::c_int);
    }
    strcpy(s, b"{\0" as *const u8 as *const std::os::raw::c_char);
    match size {
        0 => {}
        1 => {
            strcat(s, *strs.offset(0 as std::os::raw::c_int as isize));
        }
        _ => {
            i = 0 as std::os::raw::c_int as size_t;
            while i < size.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
                strcat(s, *strs.offset(i as isize));
                if i < size.wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_ulong) {
                    strcat(s, b", \0" as *const u8 as *const std::os::raw::c_char);
                } else {
                    strcat(s, b" and \0" as *const u8 as *const std::os::raw::c_char);
                }
                i = i.wrapping_add(1);
                i;
            }
            strcat(s, *strs.offset(i as isize));
        }
    }
    strcat(s, b"}\0" as *const u8 as *const std::os::raw::c_char);
    return s;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut test: [*const std::os::raw::c_char; 4] = [
        b"ABC\0" as *const u8 as *const std::os::raw::c_char,
        b"DEF\0" as *const u8 as *const std::os::raw::c_char,
        b"G\0" as *const u8 as *const std::os::raw::c_char,
        b"H\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut s: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < 5 as std::os::raw::c_int as std::os::raw::c_ulong {
        s = quib(test.as_mut_ptr(), i);
        printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, s);
        free(s as *mut std::os::raw::c_void);
        i = i.wrapping_add(1);
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

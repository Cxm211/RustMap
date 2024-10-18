#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type ptrdiff_t = std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn lcs<'a1, 'a2>(
    sa: * const std::os::raw::c_char,
    sb: * const std::os::raw::c_char,
    beg: Option<&'a1 mut * mut std::os::raw::c_char>,
    end: Option<&'a2 mut * mut std::os::raw::c_char>,
) {
    let mut apos: size_t = 0;
    let mut bpos: size_t = 0;
    let mut len: ptrdiff_t = 0;
    *borrow_mut(&mut beg).unwrap() = core::ptr::null_mut();
    *borrow_mut(&mut end).unwrap() = core::ptr::null_mut();
    len = 0 as std::os::raw::c_int;
    apos = 0 as std::os::raw::c_int as size_t;
    while *sa.offset(apos as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int {
        bpos = 0 as std::os::raw::c_int as size_t;
        while *sb.offset(bpos as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int {
            if *sa.offset(apos as isize) as std::os::raw::c_int
                == *sb.offset(bpos as isize) as std::os::raw::c_int
            {
                len = 1 as std::os::raw::c_int;
                while *sa.offset(apos.wrapping_add(len as std::os::raw::c_ulong) as isize)
                    as std::os::raw::c_int != 0 as std::os::raw::c_int
                    && *sb.offset(bpos.wrapping_add(len as std::os::raw::c_ulong) as isize)
                        as std::os::raw::c_int != 0 as std::os::raw::c_int
                    && *sa.offset(apos.wrapping_add(len as std::os::raw::c_ulong) as isize)
                        as std::os::raw::c_int
                        == *sb.offset(bpos.wrapping_add(len as std::os::raw::c_ulong) as isize)
                            as std::os::raw::c_int
                {
                    len += 1;
                    len;
                }
            }
            if len as std::os::raw::c_long > (*borrow_mut(&mut end).unwrap()).offset_from(*borrow_mut(&mut beg).unwrap()) as std::os::raw::c_long {
                *borrow_mut(&mut beg).unwrap() = sa.offset(apos as isize) as *mut std::os::raw::c_char;
                *borrow_mut(&mut end).unwrap() = (*borrow_mut(&mut beg).unwrap()).offset(len as isize);
                len = 0 as std::os::raw::c_int;
            }
            bpos = bpos.wrapping_add(1);
            bpos;
        }
        apos = apos.wrapping_add(1);
        apos;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut s1: *mut std::os::raw::c_char = b"thisisatest\0" as *const u8 as *const std::os::raw::c_char
        as *mut std::os::raw::c_char;
    let mut s2: *mut std::os::raw::c_char = b"testing123testing\0" as *const u8
        as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut beg: *mut std::os::raw::c_char = core::ptr::null_mut();
    let mut end: *mut std::os::raw::c_char = core::ptr::null_mut();
    let mut it: *mut std::os::raw::c_char = core::ptr::null_mut();
    lcs(s1, s2, Some(&mut beg), Some(&mut end));
    it = beg;
    while it != end {
        putchar(*it as std::os::raw::c_int);
        it = it.offset(1);
        it;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

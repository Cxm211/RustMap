#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edit_s {
    pub c1: std::os::raw::c_char,
    pub c2: std::os::raw::c_char,
    pub n: std::os::raw::c_int,
    pub next: * mut crate::edit_s,
}
impl std::default::Default for edit_s {
    fn default() -> Self {
        edit_s {
        c1: std::os::raw::c_char::default(),
        c2: std::os::raw::c_char::default(),
        n: std::os::raw::c_int::default(),
        next: core::ptr::null_mut()
        }
    }
}

pub type edit = * mut crate::edit_s;
pub type edit_t = crate::edit_s;
#[no_mangle]
pub unsafe extern "C" fn leven(mut a: *mut std::os::raw::c_char, mut b: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut la: std::os::raw::c_int = strlen(a) as std::os::raw::c_int;
    let mut lb: std::os::raw::c_int = strlen(b) as std::os::raw::c_int;
    let mut tbl: *mut edit = malloc(
        (std::mem::size_of::<edit>() as std::os::raw::c_ulong)
            .wrapping_mul((1 as std::os::raw::c_int + la) as std::os::raw::c_ulong),
    ) as *mut edit;
    let ref mut fresh0 = *tbl.offset(0 as std::os::raw::c_int as isize);
    *fresh0 = calloc(
        ((1 as std::os::raw::c_int + la) * (1 as std::os::raw::c_int + lb)) as std::os::raw::c_ulong,
        std::mem::size_of::<edit_t>() as std::os::raw::c_ulong,
    ) as edit;
    i = 1 as std::os::raw::c_int;
    while i <= la {
        let ref mut fresh1 = *tbl.offset(i as isize);
        *fresh1 = (*tbl.offset((i - 1 as std::os::raw::c_int) as isize))
            .offset((1 as std::os::raw::c_int + lb) as isize);
        i += 1;
        i;
    }
    i = la;
    while i >= 0 as std::os::raw::c_int {
        let mut aa: *mut std::os::raw::c_char = a.offset(i as isize);
        j = lb;
        while j >= 0 as std::os::raw::c_int {
            let mut bb: *mut std::os::raw::c_char = b.offset(j as isize);
            if !(*aa == 0 && *bb == 0) {
                let mut e: _ = Some(&mut *(*tbl.offset(i as isize)).offset(j as isize));
                let mut repl: edit = &mut *(*tbl.offset((i + 1 as std::os::raw::c_int) as isize))
                    .offset((j + 1 as std::os::raw::c_int) as isize) as *mut edit_s;
                let mut dela: edit = &mut *(*tbl.offset((i + 1 as std::os::raw::c_int) as isize))
                    .offset(j as isize) as *mut edit_s;
                let mut delb: edit = &mut *(*tbl.offset(i as isize))
                    .offset((j + 1 as std::os::raw::c_int) as isize) as *mut edit_s;
                (*borrow_mut(&mut e).unwrap()).c1 = *aa;
                (*borrow_mut(&mut e).unwrap()).c2 = *bb;
                if *aa == 0 {
                    (*borrow_mut(&mut e).unwrap()).next = delb;
                    (*borrow_mut(&mut e).unwrap()).n = (*(*borrow(& e).unwrap()).next).n + 1 as std::os::raw::c_int;
                } else if *bb == 0 {
                    (*borrow_mut(&mut e).unwrap()).next = dela;
                    (*borrow_mut(&mut e).unwrap()).n = (*(*borrow(& e).unwrap()).next).n + 1 as std::os::raw::c_int;
                } else {
                    (*borrow_mut(&mut e).unwrap()).next = repl;
                    if *aa as std::os::raw::c_int == *bb as std::os::raw::c_int {
                        (*borrow_mut(&mut e).unwrap()).n = (*(*borrow_mut(&mut e).unwrap()).next).n;
                    } else {
                        if (*(*borrow(& e).unwrap()).next).n > (*delb).n {
                            (*borrow_mut(&mut e).unwrap()).next = delb;
                            (*borrow_mut(&mut e).unwrap()).c1 = 0 as std::os::raw::c_int as std::os::raw::c_char;
                        }
                        if (*(*borrow(& e).unwrap()).next).n > (*dela).n {
                            (*borrow_mut(&mut e).unwrap()).next = dela;
                            (*borrow_mut(&mut e).unwrap()).c1 = *aa;
                            (*borrow_mut(&mut e).unwrap()).c2 = 0 as std::os::raw::c_int as std::os::raw::c_char;
                        }
                        (*borrow_mut(&mut e).unwrap()).n = (*(*borrow(& e).unwrap()).next).n + 1 as std::os::raw::c_int;
                    }
                }
            }
            j -= 1;
            j;
        }
        i -= 1;
        i;
    }
    let mut p: edit = *tbl.offset(0 as std::os::raw::c_int as isize);
    printf(b"%s -> %s: %d edits\n\0" as *const u8 as *const std::os::raw::c_char, a, b, (*p).n);
    while !((*p).next).is_null() {
        if (*p).c1 as std::os::raw::c_int == (*p).c2 as std::os::raw::c_int {
            printf(b"%c\0" as *const u8 as *const std::os::raw::c_char, (*p).c1 as std::os::raw::c_int);
        } else {
            putchar('(' as i32);
            if (*p).c1 != 0 {
                putchar((*p).c1 as std::os::raw::c_int);
            }
            putchar(',' as i32);
            if (*p).c2 != 0 {
                putchar((*p).c2 as std::os::raw::c_int);
            }
            putchar(')' as i32);
        }
        p = (*p).next;
    }
    putchar('\n' as i32);
    free(*tbl.offset(0 as std::os::raw::c_int as isize) as *mut u8);
    free(tbl as *mut u8);
}
unsafe fn main_0() -> std::os::raw::c_int {
    leven(
        b"raisethysword\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        b"rosettacode\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

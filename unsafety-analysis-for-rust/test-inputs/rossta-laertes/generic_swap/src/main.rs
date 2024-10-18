#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test {
    pub a: std::os::raw::c_int,
    pub b: std::os::raw::c_int,
    pub c: std::os::raw::c_int,
}
impl std::default::Default for test {
    fn default() -> Self {
        test {
        a: std::os::raw::c_int::default(),
        b: std::os::raw::c_int::default(),
        c: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn swap(
    mut va: *mut u8,
    mut vb: *mut u8,
    mut s: size_t,
) {
    let mut t: std::os::raw::c_char = 0;
    let mut a: *mut std::os::raw::c_char = va as *mut std::os::raw::c_char;
    let mut b: *mut std::os::raw::c_char = vb as *mut std::os::raw::c_char;
    loop {
        let fresh0 = s;
        s = s.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        t = *a.offset(s as isize);
        *a.offset(s as isize) = *b.offset(s as isize);
        *b.offset(s as isize) = t;
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut t: test = {
        let mut init = test {
            a: 1 as std::os::raw::c_int,
            b: 2 as std::os::raw::c_int,
            c: 3 as std::os::raw::c_int,
        };
        init
    };
    let mut h: test = {
        let mut init = test {
            a: 4 as std::os::raw::c_int,
            b: 5 as std::os::raw::c_int,
            c: 6 as std::os::raw::c_int,
        };
        init
    };
    let mut alfa: std::os::raw::c_double = 0.45f64;
    let mut omega: std::os::raw::c_double = 9.98f64;
    let mut pt: _ = Some(&mut t);
    let mut th: _ = Some(&mut h);
    printf(b"%d %d %d\n\0" as *const u8 as *const std::os::raw::c_char, t.a, t.b, t.c);
    let mut _T: test = t;
    t = h;
    h = _T;
    printf(b"%d %d %d\n\0" as *const u8 as *const std::os::raw::c_char, t.a, t.b, t.c);
    printf(b"%d %d %d\n\0" as *const u8 as *const std::os::raw::c_char, h.a, h.b, h.c);
    printf(b"%lf\n\0" as *const u8 as *const std::os::raw::c_char, alfa);
    let mut _T_0: std::os::raw::c_double = alfa;
    alfa = omega;
    omega = _T_0;
    printf(b"%lf\n\0" as *const u8 as *const std::os::raw::c_char, alfa);
    printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, (*borrow_mut(&mut pt).unwrap()).a);
    let mut _T_1: _ = borrow_mut(&mut pt);
    pt = borrow_mut(&mut th);
    th = borrow_mut(&mut _T_1);
    printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, (*borrow_mut(&mut pt).unwrap()).a);
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

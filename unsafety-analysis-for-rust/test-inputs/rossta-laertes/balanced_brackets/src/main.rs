#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
#[no_mangle]
pub unsafe extern "C" fn isBal(
    mut s: *const std::os::raw::c_char,
    mut l: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut c: std::os::raw::c_int = 0 as std::os::raw::c_int;
    loop {
        let fresh0 = l;
        l = l - 1;
        if !(fresh0 != 0) {
            break;
        }
        if *s.offset(l as isize) as std::os::raw::c_int == ']' as i32 {
            c += 1;
            c;
        } else {
            if !(*s.offset(l as isize) as std::os::raw::c_int == '[' as i32) {
                continue;
            }
            c -= 1;
            if c < 0 as std::os::raw::c_int {
                break;
            }
        }
    }
    return (c == 0) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn shuffle(mut s: *mut std::os::raw::c_char, mut h: std::os::raw::c_int) {
    let mut x: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = h;
    loop {
        let fresh1 = i;
        i = i - 1;
        if !(fresh1 != 0) {
            break;
        }
        x = rand() % h;
        t = *s.offset(x as isize) as std::os::raw::c_int;
        *s.offset(x as isize) = *s.offset(i as isize);
        *s.offset(i as isize) = t as std::os::raw::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn genSeq(mut s: *mut std::os::raw::c_char, mut n: std::os::raw::c_int) {
    if n != 0 {
        memset(s as *mut u8, '[' as i32, n as std::os::raw::c_ulong);
        memset(
            s.offset(n as isize) as *mut u8,
            ']' as i32,
            n as std::os::raw::c_ulong,
        );
        shuffle(s, n * 2 as std::os::raw::c_int);
    }
    *s.offset((n * 2 as std::os::raw::c_int) as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn doSeq(mut n: std::os::raw::c_int) {
    let mut s: [std::os::raw::c_char; 64] = [0; 64];
    let mut o: *const std::os::raw::c_char = b"False\0" as *const u8 as *const std::os::raw::c_char;
    genSeq(s.as_mut_ptr(), n);
    if isBal(s.as_mut_ptr(), n * 2 as std::os::raw::c_int) != 0 {
        o = b"True\0" as *const u8 as *const std::os::raw::c_char;
    }
    printf(b"'%s': %s\n\0" as *const u8 as *const std::os::raw::c_char, s.as_mut_ptr(), o);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while n < 9 as std::os::raw::c_int {
        let fresh2 = n;
        n = n + 1;
        doSeq(fresh2);
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

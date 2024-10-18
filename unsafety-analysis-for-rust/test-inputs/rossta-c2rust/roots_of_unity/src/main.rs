#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
}
unsafe fn main_0() -> libc::c_int {
    let mut a: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut PI2: libc::c_double = atan2(
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ) * 8 as libc::c_int as libc::c_double;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    n = 1 as libc::c_int;
    while n < 10 as libc::c_int {
        i = 0 as libc::c_int;
        while i < n {
            s = 0 as libc::c_int as libc::c_double;
            c = s;
            if i == 0 {
                c = 1 as libc::c_int as libc::c_double;
            } else if n == 4 as libc::c_int * i {
                s = 1 as libc::c_int as libc::c_double;
            } else if n == 2 as libc::c_int * i {
                c = -(1 as libc::c_int) as libc::c_double;
            } else if 3 as libc::c_int * n == 4 as libc::c_int * i {
                s = -(1 as libc::c_int) as libc::c_double;
            } else {
                a = i as libc::c_double * PI2 / n as libc::c_double;
                c = cos(a);
                s = sin(a);
            }
            if c != 0. {
                printf(b"%.2g\0" as *const u8 as *const libc::c_char, c);
            }
            printf(
                if s == 1 as libc::c_int as libc::c_double {
                    b"i\0" as *const u8 as *const libc::c_char
                } else if s == -(1 as libc::c_int) as libc::c_double {
                    b"-i\0" as *const u8 as *const libc::c_char
                } else if s != 0. {
                    b"%+.2gi\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                s,
            );
            printf(
                if i == n - 1 as libc::c_int {
                    b"\n\0" as *const u8 as *const libc::c_char
                } else {
                    b",  \0" as *const u8 as *const libc::c_char
                },
            );
            i += 1;
            i;
        }
        n += 1;
        n;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

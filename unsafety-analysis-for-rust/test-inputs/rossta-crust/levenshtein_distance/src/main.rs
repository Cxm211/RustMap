#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn levenshtein(mut s: *const i8, mut ls: i32, mut t: *const i8, mut lt: i32) -> i32 {
    unsafe {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        if ls == 0 {
            return lt;
        }
        if lt == 0 {
            return ls;
        }
        if *s.offset((ls - 1i32) as isize) as i32 == *t.offset((lt - 1i32) as isize) as i32 {
            return levenshtein(s, ls - 1, t, lt - 1);
        }
        a = levenshtein(s, ls - 1, t, lt - 1);
        b = levenshtein(s, ls, t, lt - 1);
        c = levenshtein(s, ls - 1, t, lt);
        if a > b {
            a = b;
        }
        if a > c {
            a = c;
        }
        return a + 1;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut s1: *const i8 = b"rosettacode\0" as *const u8 as *const i8;
        let mut s2: *const i8 = b"raisethysword\0" as *const u8 as *const i8;
        printf(
            b"distance between `%s' and `%s': %d\n\0" as *const u8 as *const i8,
            s1,
            s2,
            levenshtein(s1, strlen(s1) as i32, s2, strlen(s2) as i32),
        );
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

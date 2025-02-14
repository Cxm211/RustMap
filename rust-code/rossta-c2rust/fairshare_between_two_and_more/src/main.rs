#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn turn(mut base: std::os::raw::c_int, mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while n != 0 as std::os::raw::c_int {
        let mut rem: std::os::raw::c_int = n % base;
        n = n / base;
        sum += rem;
    }
    return sum % base;
}
#[no_mangle]
pub unsafe extern "C" fn fairshare(mut base: std::os::raw::c_int, mut count: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    printf(b"Base %2d:\0" as *const u8 as *const std::os::raw::c_char, base);
    i = 0 as std::os::raw::c_int;
    while i < count {
        let mut t: std::os::raw::c_int = turn(base, i);
        printf(b" %2d\0" as *const u8 as *const std::os::raw::c_char, t);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn turnCount(mut base: std::os::raw::c_int, mut count: std::os::raw::c_int) {
    let mut cnt: *mut std::os::raw::c_int = calloc(
        base as std::os::raw::c_ulong,
        ::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut minTurn: std::os::raw::c_int = 0;
    let mut maxTurn: std::os::raw::c_int = 0;
    let mut portion: std::os::raw::c_int = 0;
    if cnt.is_null() {
        printf(
            b"Failed to allocate space to determine the spread of turns.\n\0"
                as *const u8 as *const std::os::raw::c_char,
        );
        return;
    }
    i = 0 as std::os::raw::c_int;
    while i < count {
        let mut t: std::os::raw::c_int = turn(base, i);
        let ref mut fresh0 = *cnt.offset(t as isize);
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    minTurn = 2147483647 as std::os::raw::c_int;
    maxTurn = -(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int;
    portion = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < base {
        if *cnt.offset(i as isize) > 0 as std::os::raw::c_int {
            portion += 1;
            portion;
        }
        if *cnt.offset(i as isize) < minTurn {
            minTurn = *cnt.offset(i as isize);
        }
        if *cnt.offset(i as isize) > maxTurn {
            maxTurn = *cnt.offset(i as isize);
        }
        i += 1;
        i;
    }
    printf(b"  With %d people: \0" as *const u8 as *const std::os::raw::c_char, base);
    if 0 as std::os::raw::c_int == minTurn {
        printf(b"Only %d have a turn\n\0" as *const u8 as *const std::os::raw::c_char, portion);
    } else if minTurn == maxTurn {
        printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, minTurn);
    } else {
        printf(b"%d or %d\n\0" as *const u8 as *const std::os::raw::c_char, minTurn, maxTurn);
    }
    free(cnt as *mut std::os::raw::c_void);
}
unsafe fn main_0() -> std::os::raw::c_int {
    fairshare(2 as std::os::raw::c_int, 25 as std::os::raw::c_int);
    fairshare(3 as std::os::raw::c_int, 25 as std::os::raw::c_int);
    fairshare(5 as std::os::raw::c_int, 25 as std::os::raw::c_int);
    fairshare(11 as std::os::raw::c_int, 25 as std::os::raw::c_int);
    printf(
        b"How many times does each get a turn in 50000 iterations?\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    turnCount(191 as std::os::raw::c_int, 50000 as std::os::raw::c_int);
    turnCount(1377 as std::os::raw::c_int, 50000 as std::os::raw::c_int);
    turnCount(49999 as std::os::raw::c_int, 50000 as std::os::raw::c_int);
    turnCount(50000 as std::os::raw::c_int, 50000 as std::os::raw::c_int);
    turnCount(50001 as std::os::raw::c_int, 50000 as std::os::raw::c_int);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

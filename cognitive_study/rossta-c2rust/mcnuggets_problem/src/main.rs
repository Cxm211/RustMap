#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut sixes: libc::c_int = 0;
    let mut nines: libc::c_int = 0;
    let mut twenties: libc::c_int = 0;
    's_3: while i < 100 as libc::c_int {
        sixes = 0 as libc::c_int;
        while (sixes * 6 as libc::c_int) < i {
            if sixes * 6 as libc::c_int == i {
                i += 1;
                i;
                continue 's_3;
            } else {
                nines = 0 as libc::c_int;
                while (nines * 9 as libc::c_int) < i {
                    if sixes * 6 as libc::c_int + nines * 9 as libc::c_int == i {
                        i += 1;
                        i;
                        continue 's_3;
                    } else {
                        twenties = 0 as libc::c_int;
                        while (twenties * 20 as libc::c_int) < i {
                            if sixes * 6 as libc::c_int + nines * 9 as libc::c_int
                                + twenties * 20 as libc::c_int == i
                            {
                                i += 1;
                                i;
                                continue 's_3;
                            } else {
                                twenties += 1;
                                twenties;
                            }
                        }
                        nines += 1;
                        nines;
                    }
                }
                sixes += 1;
                sixes;
            }
        }
        max = i;
        i += 1;
        i;
    }
    printf(
        b"Maximum non-McNuggets number is %d\n\0" as *const u8 as *const libc::c_char,
        max,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

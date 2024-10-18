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
extern "C" {}
fn main_0() -> i32 {
    let mut max: i32 = 0;
    let mut i: i32 = 0;
    let mut sixes: i32 = 0;
    let mut nines: i32 = 0;
    let mut twenties: i32 = 0;
    's_3: while i < 100 {
        sixes = 0;
        while (sixes * 6) < i {
            if sixes * 6 == i {
                i += 1;
                i;
                continue 's_3;
            } else {
                nines = 0;
                while (nines * 9) < i {
                    if sixes * 6 + nines * 9 == i {
                        i += 1;
                        i;
                        continue 's_3;
                    } else {
                        twenties = 0;
                        while (twenties * 20) < i {
                            if sixes * 6 + nines * 9 + twenties * 20 == i {
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
    print!("Maximum non-McNuggets number is {}\n", max);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

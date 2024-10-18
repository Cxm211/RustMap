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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn turn(mut base: i32, mut n: i32) -> i32 {
    let mut sum: i32 = 0;
    while n != 0 {
        let mut rem: i32 = n % base;
        n = n / base;
        sum += rem;
    }
    return sum % base;
}

#[no_mangle]
pub extern "C" fn fairshare(mut base: i32, mut count: i32) {
    let mut i: i32 = 0;
    print!("Base {:2}:", base);
    i = 0;
    while i < count {
        let mut t: i32 = turn(base, i);
        print!(" {:2}", t);
        i += 1;
        i;
    }
    print!("\n");
}

#[no_mangle]
pub extern "C" fn turnCount(mut base: i32, mut count: i32) {
    unsafe {
        let mut cnt: *mut i32 =
            calloc(base as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        let mut i: i32 = 0;
        let mut minTurn: i32 = 0;
        let mut maxTurn: i32 = 0;
        let mut portion: i32 = 0;
        if cnt.is_null() {
            print!("Failed to allocate space to determine the spread of turns.\n");
            return;
        }
        i = 0;
        while i < count {
            let mut t: i32 = turn(base, i);
            let ref mut fresh0 = *cnt.offset(t as isize);
            *fresh0 += 1;
            *fresh0;
            i += 1;
            i;
        }
        minTurn = 2147483647;
        maxTurn = -2147483647 - 1;
        portion = 0;
        i = 0;
        while i < base {
            if *cnt.offset(i as isize) > 0 {
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
        print!("  With {} people: ", base);
        if 0 == minTurn {
            print!("Only {} have a turn\n", portion);
        } else if minTurn == maxTurn {
            print!("{}\n", minTurn);
        } else {
            print!("{} or {}\n", minTurn, maxTurn);
        }
        free(cnt as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    fairshare(2, 25);
    fairshare(3, 25);
    fairshare(5, 25);
    fairshare(11, 25);
    print!("How many times does each get a turn in 50000 iterations?\n");
    turnCount(191, 50000);
    turnCount(1377, 50000);
    turnCount(49999, 50000);
    turnCount(50000, 50000);
    turnCount(50001, 50000);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

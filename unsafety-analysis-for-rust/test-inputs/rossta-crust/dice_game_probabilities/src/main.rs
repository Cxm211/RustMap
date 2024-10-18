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
#[no_mangle]
pub extern "C" fn ipow(x: u32, y: u32) -> u64 {
    let mut result: u64 = 1;
    let mut i: u32 = 1;
    while i <= y {
        result = (result).wrapping_mul(x as u64) as u64;
        i = i.wrapping_add(1);
        i;
    }
    return result;
}

#[no_mangle]
pub extern "C" fn min(x: u32, y: u32) -> u32 {
    return if x < y { x } else { y };
}

#[no_mangle]
pub extern "C" fn throw_die(n_sides: u32, n_dice: u32, s: u32, mut counts: *mut u32) {
    unsafe {
        if n_dice == 0 {
            let ref mut fresh0 = *counts.offset(s as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            *fresh0;
            return;
        }
        let mut i: u32 = 1;
        while i < n_sides.wrapping_add(1) {
            throw_die(n_sides, n_dice.wrapping_sub(1), s.wrapping_add(i), counts);
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn beating_probability(
    n_sides1: u32,
    n_dice1: u32,
    n_sides2: u32,
    n_dice2: u32,
) -> f64 {
    let len1: u32 = n_sides1.wrapping_add(1).wrapping_mul(n_dice1);
    let vla = len1 as usize;
    let mut C1: Vec<u32> = ::std::vec::from_elem(0, vla);
    let mut i: u32 = 0;
    unsafe {
        while i < len1 {
            *C1.as_mut_ptr().offset(i as isize) = 0;
            i = i.wrapping_add(1);
            i;
        }
    }
    throw_die(n_sides1, n_dice1, 0, C1.as_mut_ptr());
    let len2: u32 = n_sides2.wrapping_add(1).wrapping_mul(n_dice2);
    let vla_0 = len2 as usize;
    let mut C2: Vec<u32> = ::std::vec::from_elem(0, vla_0);
    let mut j: u32 = 0;
    unsafe {
        while j < len2 {
            *C2.as_mut_ptr().offset(j as isize) = 0;
            j = j.wrapping_add(1);
            j;
        }
    }
    throw_die(n_sides2, n_dice2, 0, C2.as_mut_ptr());
    let p12: f64 = (ipow(n_sides1, n_dice1)).wrapping_mul(ipow(n_sides2, n_dice2)) as f64;
    let mut tot: f64 = 0 as f64;
    let mut i_0: u32 = 0;
    unsafe {
        while i_0 < len1 {
            let mut j_0: u32 = 0;
            while j_0 < min(i_0, len2) {
                tot += *C1.as_mut_ptr().offset(i_0 as isize) as f64
                    * *C2.as_mut_ptr().offset(j_0 as isize) as f64
                    / p12;
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    return tot;
}

fn main_0() -> i32 {
    print!("{:1.16}\n", beating_probability(4, 9, 6, 6));
    print!("{:1.16}\n", beating_probability(10, 5, 7, 6));
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

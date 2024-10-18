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
pub static mut total: u64 = 0;
#[no_mangle]
pub static mut prim: u64 = 0;
#[no_mangle]
pub static mut max_peri: u64 = 0;
#[no_mangle]
pub static mut U: [[u64; 9]; 3] = [
    [
        1,
        (-2) as i64 as u64,
        2,
        2,
        (-1) as i64 as u64,
        2,
        2,
        (-2) as i64 as u64,
        3,
    ],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [
        (-1) as i64 as u64,
        2,
        2,
        (-2) as i64 as u64,
        1,
        2,
        (-2) as i64 as u64,
        2,
        3,
    ],
];
#[no_mangle]
pub extern "C" fn new_tri(mut in_0: *mut u64) {
    unsafe {
        let mut i: i32 = 0;
        let mut t: [u64; 3] = [0; 3];
        let mut p: u64 = (*in_0.offset(0 as isize))
            .wrapping_add(*in_0.offset(1 as isize))
            .wrapping_add(*in_0.offset(2 as isize));
        if p > max_peri {
            return;
        }
        prim = prim.wrapping_add(1);
        prim;
        total = (total as u64).wrapping_add(max_peri.wrapping_div(p)) as u64;
        i = 0;
        while i < 3 {
            t[0 as usize] = (U[i as usize][0 as usize])
                .wrapping_mul(*in_0.offset(0 as isize))
                .wrapping_add((U[i as usize][1 as usize]).wrapping_mul(*in_0.offset(1 as isize)))
                .wrapping_add((U[i as usize][2 as usize]).wrapping_mul(*in_0.offset(2 as isize)));
            t[1 as usize] = (U[i as usize][3 as usize])
                .wrapping_mul(*in_0.offset(0 as isize))
                .wrapping_add((U[i as usize][4 as usize]).wrapping_mul(*in_0.offset(1 as isize)))
                .wrapping_add((U[i as usize][5 as usize]).wrapping_mul(*in_0.offset(2 as isize)));
            t[2 as usize] = (U[i as usize][6 as usize])
                .wrapping_mul(*in_0.offset(0 as isize))
                .wrapping_add((U[i as usize][7 as usize]).wrapping_mul(*in_0.offset(1 as isize)))
                .wrapping_add((U[i as usize][8 as usize]).wrapping_mul(*in_0.offset(2 as isize)));
            new_tri(t.as_mut_ptr());
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut seed: [u64; 3] = [3, 4, 5];
    unsafe {
        max_peri = 10;
        while max_peri <= 100000000 {
            prim = 0;
            total = prim;
            new_tri(seed.as_mut_ptr());
            print!(
                "Up to {}: {} triples, {} primitives.\n",
                max_peri, total, prim
            );
            max_peri = (max_peri as u64).wrapping_mul(10) as u64;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
    let mut n: u64 = 1;
    let mut i: i32 = 0;
    while i < 30 {
        print!("{} ", n.count_ones() as i32);
        n = n.wrapping_mul(3);
        i += 1;
        i;
    }
    print!("\n");
    let mut od: [i32; 30] = [0; 30];
    let mut ne: i32 = 0;
    let mut no: i32 = 0;
    print!("evil  : ");
    let mut n_0: i32 = 0;
    while ne + no < 60 {
        if (n_0 as u32).count_ones() as i32 & 1 == 0 {
            if ne < 30 {
                print!("{} ", n_0);
                ne += 1;
                ne;
            }
        } else if no < 30 {
            let fresh0 = no;
            no = no + 1;
            od[fresh0 as usize] = n_0;
        }
        n_0 += 1;
        n_0;
    }
    print!("\n");
    print!("odious: ");
    let mut i_0: i32 = 0;
    while i_0 < 30 {
        print!("{} ", od[i_0 as usize]);
        i_0 += 1;
        i_0;
    }
    print!("\n");
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
pub static mut a1: [i8; 3] = ['a' as i8, 'b' as i8, 'c' as i8];
#[no_mangle]
pub static mut a2: [i8; 3] = ['A' as i8, 'B' as i8, 'C' as i8];
#[no_mangle]
pub static mut a3: [i32; 3] = [1, 2, 3];
fn main_0() -> i32 {
    let mut i: i32 = 0;
    unsafe {
        while i < 3 {
            print!(
                "{}{}{}\n",
                a1[i as usize] as i32, a2[i as usize] as i32, a3[i as usize]
            );
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
    let mut one: i32 = 1;
    print!(
        "word size = {} bits\n",
        8u64.wrapping_mul(::core::mem::size_of::<u64>() as u64) as i32
    );
    if *(&mut one as *mut i32 as *mut i8) != 0 {
        print!("little endian\n");
    } else {
        print!("big endian\n");
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

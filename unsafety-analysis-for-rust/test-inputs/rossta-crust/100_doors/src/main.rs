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
    let mut is_open: [i8; 100] = [0; 100];
    let mut pass: i32 = 0;
    let mut door: i32 = 0;
    pass = 0;
    while pass < 100 {
        door = pass;
        while door < 100 {
            is_open[door as usize] = (is_open[door as usize] == 0) as i8;
            door += pass + 1;
        }
        pass += 1;
        pass;
    }
    door = 0;
    while door < 100 {
        if is_open[door as usize] as i32 != 0 {
            print!("door #{} is {}.\n", door + 1, "open\0")
        } else {
            print!("door #{} is {}.\n", door + 1, "closed\0")
        };
        door += 1;
        door;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

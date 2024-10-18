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
use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

//use c2rust_out::*;
extern "C" {
    fn srand(__seed: u32);
    fn rand() -> i32;
}
fn main_0() -> i32 {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut k: u32 = 0;
    let mut choice: u32 = 0;
    let mut winsbyswitch: u32 = 0;
    let mut door: [u32; 3] = [0; 3];
    unsafe {
        srand(rust_time(None) as u32);
    }
    i = 0;
    unsafe {
        while i < 3000000 {
            door[0 as usize] = (if rand() % 2 == 0 { 1i32 } else { 0 }) as u32;
            if door[0 as usize] != 0 {
                door[2 as usize] = 0;
                door[1 as usize] = door[2 as usize];
            } else {
                door[1 as usize] = (if rand() % 2 == 0 { 1i32 } else { 0 }) as u32;
                door[2 as usize] = (if door[1 as usize] == 0 { 1 } else { 0 }) as u32;
            }
            choice = (rand() % 3i32) as u32;
            if door[choice.wrapping_add(1).wrapping_rem(3) as usize] == 0
                && door[choice.wrapping_add(2).wrapping_rem(3) as usize] != 0
                || door[choice.wrapping_add(2).wrapping_rem(3) as usize] == 0
                    && door[choice.wrapping_add(1).wrapping_rem(3) as usize] != 0
            {
                winsbyswitch = winsbyswitch.wrapping_add(1);
                winsbyswitch;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    print!(
        "\nAfter {} games, I won {} by switching.  That is {}%. ",
        3000000,
        winsbyswitch,
        winsbyswitch as libc::c_float as f64 * 100.0f64 / i as libc::c_float as f64
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

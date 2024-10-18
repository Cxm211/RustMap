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
pub static mut one: u64 = 1;
#[no_mangle]
pub extern "C" fn comb(mut pool: i32, mut need: i32, mut chosen: u64, mut at: i32) {
    if pool < need + at {
        return;
    }
    unsafe {
        if need == 0 {
            at = 0;
            while at < pool {
                if chosen & one << at != 0 {
                    print!("{} ", at);
                }
                at += 1;
                at;
            }
            print!("\n");
            return;
        }
        comb(pool, need - 1, chosen | one << at, at + 1);
    }
    comb(pool, need, chosen, at + 1);
}

fn main_0() -> i32 {
    comb(5, 3, 0, 0);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

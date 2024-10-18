#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type marker = std::os::raw::c_ulong;
#[no_mangle]
pub static mut one: marker = 1 as std::os::raw::c_int as marker;
#[no_mangle]
pub unsafe extern "C" fn comb(
    mut pool: std::os::raw::c_int,
    mut need: std::os::raw::c_int,
    mut chosen: marker,
    mut at: std::os::raw::c_int,
) {
    if pool < need + at {
        return;
    }
    if need == 0 {
        at = 0 as std::os::raw::c_int;
        while at < pool {
            if chosen & one << at != 0 {
                printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, at);
            }
            at += 1;
            at;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        return;
    }
    comb(pool, need - 1 as std::os::raw::c_int, chosen | one << at, at + 1 as std::os::raw::c_int);
    comb(pool, need, chosen, at + 1 as std::os::raw::c_int);
}
unsafe fn main_0() -> std::os::raw::c_int {
    comb(
        5 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        0 as std::os::raw::c_int as marker,
        0 as std::os::raw::c_int,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

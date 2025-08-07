#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn rand() -> std::os::raw::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = std::os::raw::c_long;
pub type time_t = __time_t;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_uint = 0;
    let mut j: std::os::raw::c_uint = 0;
    let mut k: std::os::raw::c_uint = 0;
    let mut choice: std::os::raw::c_uint = 0;
    let mut winsbyswitch: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut door: [std::os::raw::c_uint; 3] = [0; 3];
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < 3000000 as std::os::raw::c_int as std::os::raw::c_uint {
        door[0 as std::os::raw::c_int
            as usize] = (if rand() % 2 as std::os::raw::c_int == 0 {
            1 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        }) as std::os::raw::c_uint;
        if door[0 as std::os::raw::c_int as usize] != 0 {
            door[2 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            door[1 as std::os::raw::c_int as usize] = door[2 as std::os::raw::c_int as usize];
        } else {
            door[1 as std::os::raw::c_int
                as usize] = (if rand() % 2 as std::os::raw::c_int == 0 {
                1 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            }) as std::os::raw::c_uint;
            door[2 as std::os::raw::c_int
                as usize] = (if door[1 as std::os::raw::c_int as usize] == 0 {
                1 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            }) as std::os::raw::c_uint;
        }
        choice = (rand() % 3 as std::os::raw::c_int) as std::os::raw::c_uint;
        if door[choice
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint) as usize] == 0
            && door[choice
                .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint) as usize] != 0
            || door[choice
                .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint) as usize] == 0
                && door[choice
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint) as usize] != 0
        {
            winsbyswitch = winsbyswitch.wrapping_add(1);
            winsbyswitch;
        }
        i = i.wrapping_add(1);
        i;
    }
    printf(
        b"\nAfter %u games, I won %u by switching.  That is %f%%. \0" as *const u8
            as *const std::os::raw::c_char,
        3000000 as std::os::raw::c_int,
        winsbyswitch,
        winsbyswitch as std::os::raw::c_float as std::os::raw::c_double * 100.0f64
            / i as std::os::raw::c_float as std::os::raw::c_double,
    );
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

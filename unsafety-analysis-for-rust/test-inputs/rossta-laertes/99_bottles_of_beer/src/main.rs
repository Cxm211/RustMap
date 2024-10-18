#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0;
    n = 99 as std::os::raw::c_int;
    while n > 2 as std::os::raw::c_int {
        printf(
            b"%d bottles of beer on the wall, %d bottles of beer.\nTake one down and pass it around, %d bottles of beer on the wall.\n\n\0"
                as *const u8 as *const std::os::raw::c_char,
            n,
            n,
            n - 1 as std::os::raw::c_int,
        );
        n -= 1;
        n;
    }
    printf(
        b"2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake one down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n\0"
            as *const u8 as *const std::os::raw::c_char,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

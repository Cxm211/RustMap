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
extern "C" {
    fn puts(__s: *const i8) -> i32;
}
fn main_0() -> i32 {
    unsafe {
        let mut object: *mut i8 = 0 as *mut i8;
        if object.is_null() {
            puts(b"object is null\0" as *const u8 as *const i8);
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

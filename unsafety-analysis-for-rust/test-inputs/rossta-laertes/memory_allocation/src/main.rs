#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn free(_: *mut u8);
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut ints: *mut std::os::raw::c_int = malloc(
        (std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(100 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    ints = realloc(
        ints as *mut u8,
        (std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul((100 as std::os::raw::c_int + 1 as std::os::raw::c_int) as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut int2: *mut std::os::raw::c_int = calloc(
        100 as std::os::raw::c_int as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    free(ints as *mut u8);
    free(int2 as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

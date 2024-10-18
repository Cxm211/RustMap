#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn log10(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
pub type __uint32_t = std::os::raw::c_uint;
pub type size_t = std::os::raw::c_ulong;
pub type uint32_t = std::os::raw::c_uint;
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < 20 as std::os::raw::c_int as std::os::raw::c_ulong {
        let mut binstr: *mut std::os::raw::c_char = bin(i as uint32_t);
        printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, binstr);
        free(binstr as *mut u8);
        i = i.wrapping_add(1);
        i;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn bin(mut x: uint32_t) -> *mut std::os::raw::c_char {
    let mut bits: size_t = (if x == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        1 as std::os::raw::c_int as std::os::raw::c_double
    } else {
        log10(x as std::os::raw::c_double) / log10(2 as std::os::raw::c_int as std::os::raw::c_double)
            + 1 as std::os::raw::c_int as std::os::raw::c_double
    }) as size_t;
    let mut ret: *mut std::os::raw::c_char = malloc(
        bits
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < bits {
        *ret
            .offset(
                bits.wrapping_sub(i).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            ) = (if x & 1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            '1' as i32
        } else {
            '0' as i32
        }) as std::os::raw::c_char;
        x >>= 1 as std::os::raw::c_int;
        i = i.wrapping_add(1);
        i;
    }
    *ret.offset(bits as isize) = '\0' as i32 as std::os::raw::c_char;
    return ret;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

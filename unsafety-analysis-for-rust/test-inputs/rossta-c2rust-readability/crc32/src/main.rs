#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type __uint8_t = std::os::raw::c_uchar;
pub type __uint32_t = std::os::raw::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn rc_crc32(
    mut crc: uint32_t,
    mut buf: *const std::os::raw::c_char,
    mut len: size_t,
) -> uint32_t {
    static mut table: [uint32_t; 256] = [0; 256];
    static mut have_table: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut rem: uint32_t = 0;
    let mut octet: uint8_t = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut p: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut q: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if have_table == 0 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int;
        while i < 256 as std::os::raw::c_int {
            rem = i as uint32_t;
            j = 0 as std::os::raw::c_int;
            while j < 8 as std::os::raw::c_int {
                if rem & 1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                    rem >>= 1 as std::os::raw::c_int;
                    rem ^= 0xedb88320 as std::os::raw::c_uint;
                } else {
                    rem >>= 1 as std::os::raw::c_int;
                }
                j += 1;
                j;
            }
            table[i as usize] = rem;
            i += 1;
            i;
        }
        have_table = 1 as std::os::raw::c_int;
    }
    crc = !crc;
    q = buf.offset(len as isize);
    p = buf;
    while p < q {
        octet = *p as uint8_t;
        crc = crc >> 8 as std::os::raw::c_int
            ^ table[(crc & 0xff as std::os::raw::c_int as std::os::raw::c_uint ^ octet as std::os::raw::c_uint)
                as usize];
        p = p.offset(1);
        p;
    }
    return !crc;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut s: *const std::os::raw::c_char = b"The quick brown fox jumps over the lazy dog\0"
        as *const u8 as *const std::os::raw::c_char;
    printf(
        b"%X\n\0" as *const u8 as *const std::os::raw::c_char,
        rc_crc32(0 as std::os::raw::c_int as uint32_t, s, strlen(s)),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

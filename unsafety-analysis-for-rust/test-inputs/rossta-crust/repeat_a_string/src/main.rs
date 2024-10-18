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
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn string_repeat(mut n: i32, mut s: *const i8) -> *mut i8 {
    unsafe {
        let mut slen: u64 = strlen(s);
        let mut dest: *mut i8 = malloc((n as u64).wrapping_mul(slen).wrapping_add(1)) as *mut i8;
        let mut i: i32 = 0;
        let mut p: *mut i8 = 0 as *mut i8;
        i = 0;
        p = dest;
        while i < n {
            memcpy(p as *mut libc::c_void, s as *const libc::c_void, slen);
            i += 1;
            i;
            p = p.offset(slen as isize);
        }
        *p = '\0' as i8;
        return dest;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut result: *mut i8 = string_repeat(5, b"ha\0" as *const u8 as *const i8);
        puts(result);
        free(result as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

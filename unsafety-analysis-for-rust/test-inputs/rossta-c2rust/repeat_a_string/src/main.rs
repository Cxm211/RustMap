#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn free(_: *mut std::os::raw::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn string_repeat(
    mut n: std::os::raw::c_int,
    mut s: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut slen: size_t = strlen(s);
    let mut dest: *mut std::os::raw::c_char = malloc(
        (n as std::os::raw::c_ulong)
            .wrapping_mul(slen)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    p = dest;
    while i < n {
        memcpy(p as *mut std::os::raw::c_void, s as *const std::os::raw::c_void, slen);
        i += 1;
        i;
        p = p.offset(slen as isize);
    }
    *p = '\0' as i32 as std::os::raw::c_char;
    return dest;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut result: *mut std::os::raw::c_char = string_repeat(
        5 as std::os::raw::c_int,
        b"ha\0" as *const u8 as *const std::os::raw::c_char,
    );
    puts(result);
    free(result as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

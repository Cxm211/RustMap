#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn mmap(
        __addr: *mut std::os::raw::c_void,
        __len: size_t,
        __prot: std::os::raw::c_int,
        __flags: std::os::raw::c_int,
        __fd: std::os::raw::c_int,
        __offset: __off_t,
    ) -> *mut std::os::raw::c_void;
    fn munmap(__addr: *mut std::os::raw::c_void, __len: size_t) -> std::os::raw::c_int;
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
#[no_mangle]
pub unsafe extern "C" fn test(mut a: std::os::raw::c_int, mut b: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut code: [std::os::raw::c_char; 9] = [
        0x8b as std::os::raw::c_int as std::os::raw::c_char,
        0x44 as std::os::raw::c_int as std::os::raw::c_char,
        0x24 as std::os::raw::c_int as std::os::raw::c_char,
        0x4 as std::os::raw::c_int as std::os::raw::c_char,
        0x3 as std::os::raw::c_int as std::os::raw::c_char,
        0x44 as std::os::raw::c_int as std::os::raw::c_char,
        0x24 as std::os::raw::c_int as std::os::raw::c_char,
        0x8 as std::os::raw::c_int as std::os::raw::c_char,
        0xc3 as std::os::raw::c_int as std::os::raw::c_char,
    ];
    let mut buf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut c: std::os::raw::c_int = 0;
    buf = mmap(
        0 as *mut std::os::raw::c_void,
        ::core::mem::size_of::<[std::os::raw::c_char; 9]>() as std::os::raw::c_ulong,
        0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int | 0x4 as std::os::raw::c_int,
        0x2 as std::os::raw::c_int | 0x20 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        0 as std::os::raw::c_int as __off_t,
    );
    memcpy(
        buf,
        code.as_mut_ptr() as *const std::os::raw::c_void,
        ::core::mem::size_of::<[std::os::raw::c_char; 9]>() as std::os::raw::c_ulong,
    );
    c = (::core::mem::transmute::<
        *mut std::os::raw::c_void,
        Option::<unsafe extern "C" fn(std::os::raw::c_int, std::os::raw::c_int) -> std::os::raw::c_int>,
    >(buf))
        .expect("non-null function pointer")(a, b);
    munmap(buf, ::core::mem::size_of::<[std::os::raw::c_char; 9]>() as std::os::raw::c_ulong);
    return c;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        test(7 as std::os::raw::c_int, 12 as std::os::raw::c_int),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

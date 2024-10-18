#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn asctime(__tp: *const tm) -> *mut std::os::raw::c_char;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: std::os::raw::c_int,
    pub tm_min: std::os::raw::c_int,
    pub tm_hour: std::os::raw::c_int,
    pub tm_mday: std::os::raw::c_int,
    pub tm_mon: std::os::raw::c_int,
    pub tm_year: std::os::raw::c_int,
    pub tm_wday: std::os::raw::c_int,
    pub tm_yday: std::os::raw::c_int,
    pub tm_isdst: std::os::raw::c_int,
    pub tm_gmtoff: std::os::raw::c_long,
    pub tm_zone: *const std::os::raw::c_char,
}
impl std::default::Default for tm {
    fn default() -> Self {
        tm {
        tm_sec: std::os::raw::c_int::default(),
        tm_min: std::os::raw::c_int::default(),
        tm_hour: std::os::raw::c_int::default(),
        tm_mday: std::os::raw::c_int::default(),
        tm_mon: std::os::raw::c_int::default(),
        tm_year: std::os::raw::c_int::default(),
        tm_wday: std::os::raw::c_int::default(),
        tm_yday: std::os::raw::c_int::default(),
        tm_isdst: std::os::raw::c_int::default(),
        tm_gmtoff: std::os::raw::c_long::default(),
        tm_zone: 0 as * const std::os::raw::c_char
        }
    }
}

unsafe fn main_0() -> std::os::raw::c_int {
    let mut t: time_t = 0 as std::os::raw::c_int as time_t;
    printf(b"%s\0" as *const u8 as *const std::os::raw::c_char, asctime(gmtime(&mut t)));
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut std::os::raw::c_char,
        __maxsize: size_t,
        __format: *const std::os::raw::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
}
pub type size_t = std::os::raw::c_ulong;
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
    let mut buf: [std::os::raw::c_char; 50] = [0; 50];
    let mut seconds: time_t = time(0 as *mut time_t);
    let mut now: *mut tm = localtime(&mut seconds);
    let mut months: [*const std::os::raw::c_char; 12] = [
        b"January\0" as *const u8 as *const std::os::raw::c_char,
        b"February\0" as *const u8 as *const std::os::raw::c_char,
        b"March\0" as *const u8 as *const std::os::raw::c_char,
        b"April\0" as *const u8 as *const std::os::raw::c_char,
        b"May\0" as *const u8 as *const std::os::raw::c_char,
        b"June\0" as *const u8 as *const std::os::raw::c_char,
        b"July\0" as *const u8 as *const std::os::raw::c_char,
        b"August\0" as *const u8 as *const std::os::raw::c_char,
        b"September\0" as *const u8 as *const std::os::raw::c_char,
        b"October\0" as *const u8 as *const std::os::raw::c_char,
        b"November\0" as *const u8 as *const std::os::raw::c_char,
        b"December\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut days: [*const std::os::raw::c_char; 7] = [
        b"Sunday\0" as *const u8 as *const std::os::raw::c_char,
        b"Monday\0" as *const u8 as *const std::os::raw::c_char,
        b"Tuesday\0" as *const u8 as *const std::os::raw::c_char,
        b"Wednesday\0" as *const u8 as *const std::os::raw::c_char,
        b"Thursday\0" as *const u8 as *const std::os::raw::c_char,
        b"Friday\0" as *const u8 as *const std::os::raw::c_char,
        b"Saturday\0" as *const u8 as *const std::os::raw::c_char,
    ];
    printf(
        b"%d-%d-%d\n\0" as *const u8 as *const std::os::raw::c_char,
        (*now).tm_year + 1900 as std::os::raw::c_int,
        (*now).tm_mon + 1 as std::os::raw::c_int,
        (*now).tm_mday,
    );
    printf(
        b"%s, %s %d, %d\n\0" as *const u8 as *const std::os::raw::c_char,
        days[(*now).tm_wday as usize],
        months[(*now).tm_mon as usize],
        (*now).tm_mday,
        (*now).tm_year + 1900 as std::os::raw::c_int,
    );
    strftime(
        buf.as_mut_ptr(),
        50 as std::os::raw::c_int as size_t,
        b"%A, %B %e, %Y\0" as *const u8 as *const std::os::raw::c_char,
        now,
    );
    printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, buf.as_mut_ptr());
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

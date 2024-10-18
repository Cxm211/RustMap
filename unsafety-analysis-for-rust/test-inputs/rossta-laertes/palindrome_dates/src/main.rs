#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut std::os::raw::c_char,
        __maxsize: size_t,
        __format: *const std::os::raw::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
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

#[no_mangle]
pub unsafe extern "C" fn is_palindrome(mut str: *const std::os::raw::c_char) -> bool {
    let mut n: size_t = strlen(str);
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) < n {
        if *str.offset(i as isize) as std::os::raw::c_int
            != *str.offset(n.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
        n = n.wrapping_sub(1);
        n;
    }
    return 1 as std::os::raw::c_int != 0;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut timestamp: time_t = time(0 as *mut time_t);
    let seconds_per_day: std::os::raw::c_int = 24 as std::os::raw::c_int * 60 as std::os::raw::c_int
        * 60 as std::os::raw::c_int;
    let mut count: std::os::raw::c_int = 15 as std::os::raw::c_int;
    let mut str: [std::os::raw::c_char; 32] = [0; 32];
    printf(b"Next %d palindrome dates:\n\0" as *const u8 as *const std::os::raw::c_char, count);
    while count > 0 as std::os::raw::c_int {
        let mut ptr: *mut tm = gmtime(&mut timestamp);
        strftime(
            str.as_mut_ptr(),
            std::mem::size_of::<[std::os::raw::c_char; 32]>() as std::os::raw::c_ulong,
            b"%Y%m%d\0" as *const u8 as *const std::os::raw::c_char,
            ptr,
        );
        if is_palindrome(str.as_mut_ptr()) {
            strftime(
                str.as_mut_ptr(),
                std::mem::size_of::<[std::os::raw::c_char; 32]>() as std::os::raw::c_ulong,
                b"%F\0" as *const u8 as *const std::os::raw::c_char,
                ptr,
            );
            printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, str.as_mut_ptr());
            count -= 1;
            count;
        }
        timestamp += seconds_per_day as std::os::raw::c_long;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

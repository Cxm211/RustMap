#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn mktime(__tp: *mut tm) -> time_t;
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

static mut months: [*const std::os::raw::c_char; 12] = [
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
static mut long_months: [std::os::raw::c_int; 7] = [
    0 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    4 as std::os::raw::c_int,
    6 as std::os::raw::c_int,
    7 as std::os::raw::c_int,
    9 as std::os::raw::c_int,
    11 as std::os::raw::c_int,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let mut n: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut y: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut m: std::os::raw::c_int = 0;
    let mut t: tm = {
        let mut init = tm {
            tm_sec: 0 as std::os::raw::c_int,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const std::os::raw::c_char,
        };
        init
    };
    printf(b"Months with five weekends:\n\0" as *const u8 as *const std::os::raw::c_char);
    y = 1900 as std::os::raw::c_int;
    while y <= 2100 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int;
        while i < 7 as std::os::raw::c_int {
            m = long_months[i as usize];
            t.tm_year = y - 1900 as std::os::raw::c_int;
            t.tm_mon = m;
            t.tm_mday = 1 as std::os::raw::c_int;
            if mktime(&mut t) == -(1 as std::os::raw::c_int) as std::os::raw::c_long {
                printf(
                    b"Error: %d %s\n\0" as *const u8 as *const std::os::raw::c_char,
                    y,
                    months[m as usize],
                );
            } else if t.tm_wday == 5 as std::os::raw::c_int {
                printf(
                    b"  %d %s\n\0" as *const u8 as *const std::os::raw::c_char,
                    y,
                    months[m as usize],
                );
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        y += 1;
        y;
    }
    printf(b"%d total\n\0" as *const u8 as *const std::os::raw::c_char, n);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

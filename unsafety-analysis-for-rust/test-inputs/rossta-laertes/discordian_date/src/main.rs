#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
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

#[no_mangle]
pub unsafe extern "C" fn ddate(
    mut y: std::os::raw::c_int,
    mut d: std::os::raw::c_int,
) -> *mut std::os::raw::c_char {
    let mut dyear: std::os::raw::c_int = 1166 as std::os::raw::c_int + y;
    let mut result: *mut std::os::raw::c_char = malloc(
        (100 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    if y % 400 as std::os::raw::c_int == 0 as std::os::raw::c_int
        || y % 4 as std::os::raw::c_int == 0 as std::os::raw::c_int && y % 100 as std::os::raw::c_int != 0
    {
        if d == 60 as std::os::raw::c_int {
            sprintf(
                result,
                b"St. Tib's Day, YOLD %d\0" as *const u8 as *const std::os::raw::c_char,
                dyear,
            );
            return result;
        } else if d >= 60 as std::os::raw::c_int {
            d -= 1;
            d;
        }
    }
    sprintf(
        result,
        b"%s, %s %d, YOLD %d\0" as *const u8 as *const std::os::raw::c_char,
        if d % 5 as std::os::raw::c_int == 1 as std::os::raw::c_int {
            b"Sweetmorn\0" as *const u8 as *const std::os::raw::c_char
        } else if d % 5 as std::os::raw::c_int == 2 as std::os::raw::c_int {
            b"Boomtime\0" as *const u8 as *const std::os::raw::c_char
        } else if d % 5 as std::os::raw::c_int == 3 as std::os::raw::c_int {
            b"Pungenday\0" as *const u8 as *const std::os::raw::c_char
        } else if d % 5 as std::os::raw::c_int == 4 as std::os::raw::c_int {
            b"Prickle-Prickle\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"Setting Orange\0" as *const u8 as *const std::os::raw::c_char
        },
        if (if d % 73 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            d - 1 as std::os::raw::c_int
        } else {
            d
        }) / 73 as std::os::raw::c_int == 0 as std::os::raw::c_int
        {
            b"Chaos\0" as *const u8 as *const std::os::raw::c_char
        } else if (if d % 73 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            d - 1 as std::os::raw::c_int
        } else {
            d
        }) / 73 as std::os::raw::c_int == 1 as std::os::raw::c_int
        {
            b"Discord\0" as *const u8 as *const std::os::raw::c_char
        } else if (if d % 73 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            d - 1 as std::os::raw::c_int
        } else {
            d
        }) / 73 as std::os::raw::c_int == 2 as std::os::raw::c_int
        {
            b"Confusion\0" as *const u8 as *const std::os::raw::c_char
        } else if (if d % 73 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            d - 1 as std::os::raw::c_int
        } else {
            d
        }) / 73 as std::os::raw::c_int == 3 as std::os::raw::c_int
        {
            b"Bureaucracy\0" as *const u8 as *const std::os::raw::c_char
        } else {
            b"The Aftermath\0" as *const u8 as *const std::os::raw::c_char
        },
        if d % 73 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            73 as std::os::raw::c_int
        } else {
            d % 73 as std::os::raw::c_int
        },
        dyear,
    );
    return result;
}
#[no_mangle]
pub extern "C" fn day_of_year(
    mut y: std::os::raw::c_int,
    mut m: std::os::raw::c_int,
    mut d: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut month_lengths: [std::os::raw::c_int; 12] = [
        31 as std::os::raw::c_int,
        28 as std::os::raw::c_int,
        31 as std::os::raw::c_int,
        30 as std::os::raw::c_int,
        31 as std::os::raw::c_int,
        30 as std::os::raw::c_int,
        31 as std::os::raw::c_int,
        31 as std::os::raw::c_int,
        30 as std::os::raw::c_int,
        31 as std::os::raw::c_int,
        30 as std::os::raw::c_int,
        31 as std::os::raw::c_int,
    ];
    while m > 1 as std::os::raw::c_int {
        d += month_lengths[(m - 2 as std::os::raw::c_int) as usize];
        if m == 3 as std::os::raw::c_int
            && (y % 400 as std::os::raw::c_int == 0 as std::os::raw::c_int
                || y % 4 as std::os::raw::c_int == 0 as std::os::raw::c_int
                    && y % 100 as std::os::raw::c_int != 0)
        {
            d += 1;
            d;
        }
        m -= 1;
        m;
    }
    return d;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut now: time_t = 0;
    let mut now_time: *mut tm = core::ptr::null_mut();
    let mut year: std::os::raw::c_int = 0;
    let mut doy: std::os::raw::c_int = 0;
    if argc == 1 as std::os::raw::c_int {
        now = time(0 as *mut time_t);
        now_time = localtime(&mut now);
        year = (*now_time).tm_year + 1900 as std::os::raw::c_int;
        doy = (*now_time).tm_yday + 1 as std::os::raw::c_int;
    } else if argc == 4 as std::os::raw::c_int {
        year = atoi(*argv.offset(1 as std::os::raw::c_int as isize));
        doy = day_of_year(
            atoi(*argv.offset(1 as std::os::raw::c_int as isize)),
            atoi(*argv.offset(2 as std::os::raw::c_int as isize)),
            atoi(*argv.offset(3 as std::os::raw::c_int as isize)),
        );
    }
    let mut result: *mut std::os::raw::c_char = ddate(year, doy);
    puts(result);
    free(result as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

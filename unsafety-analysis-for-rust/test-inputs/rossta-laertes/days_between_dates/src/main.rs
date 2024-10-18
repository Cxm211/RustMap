#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct date {
    pub year: std::os::raw::c_int,
    pub month: std::os::raw::c_int,
    pub day: std::os::raw::c_int,
}
impl std::default::Default for date {
    fn default() -> Self {
        date {
        year: std::os::raw::c_int::default(),
        month: std::os::raw::c_int::default(),
        day: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn extractDate(mut str: *mut std::os::raw::c_char) -> date {
    return {
        let mut init = date {
            year: 1000 as std::os::raw::c_int
                * (*str.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int - '0' as i32)
                + 100 as std::os::raw::c_int
                    * (*str.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                        - '0' as i32)
                + 10 as std::os::raw::c_int
                    * (*str.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                        - '0' as i32)
                + (*str.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int - '0' as i32),
            month: 10 as std::os::raw::c_int
                * (*str.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int - '0' as i32)
                + (*str.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_int - '0' as i32),
            day: 10 as std::os::raw::c_int
                * (*str.offset(8 as std::os::raw::c_int as isize) as std::os::raw::c_int - '0' as i32)
                + (*str.offset(9 as std::os::raw::c_int as isize) as std::os::raw::c_int - '0' as i32),
        };
        init
    };
}
#[no_mangle]
pub unsafe extern "C" fn isValidDate(mut str: *mut std::os::raw::c_char) -> bool {
    let mut newDate: date = date { year: 0, month: 0, day: 0 };
    if strlen(str) != 10 as std::os::raw::c_int as std::os::raw::c_ulong
        && *str.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int != '-' as i32
        && *str.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int != '-' as i32
    {
        return 0 as std::os::raw::c_int != 0;
    }
    newDate = extractDate(str);
    if newDate.year <= 0 as std::os::raw::c_int || newDate.month <= 0 as std::os::raw::c_int
        || newDate.day <= 0 as std::os::raw::c_int || newDate.month > 12 as std::os::raw::c_int
        || newDate.month == 2 as std::os::raw::c_int && newDate.day > 29 as std::os::raw::c_int
        || (newDate.month == 1 as std::os::raw::c_int || newDate.month == 3 as std::os::raw::c_int
            || newDate.month == 5 as std::os::raw::c_int || newDate.month == 7 as std::os::raw::c_int
            || newDate.month == 8 as std::os::raw::c_int || newDate.month == 10 as std::os::raw::c_int
            || newDate.month == 12 as std::os::raw::c_int) && newDate.day > 31 as std::os::raw::c_int
        || newDate.day > 30 as std::os::raw::c_int
        || newDate.year % 4 as std::os::raw::c_int == 0 as std::os::raw::c_int
            && newDate.month == 2 as std::os::raw::c_int && newDate.month > 28 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int != 0;
    }
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub extern "C" fn diffDays(mut date1: date, mut date2: date) -> std::os::raw::c_int {
    let mut days1: std::os::raw::c_int = 0;
    let mut days2: std::os::raw::c_int = 0;
    date1.month = (date1.month + 9 as std::os::raw::c_int) % 12 as std::os::raw::c_int;
    date1.year = date1.year - date1.month / 10 as std::os::raw::c_int;
    date2.month = (date2.month + 9 as std::os::raw::c_int) % 12 as std::os::raw::c_int;
    date2.year = date2.year - date2.month / 10 as std::os::raw::c_int;
    days1 = 365 as std::os::raw::c_int * date1.year + date1.year / 4 as std::os::raw::c_int
        - date1.year / 100 as std::os::raw::c_int + date1.year / 400 as std::os::raw::c_int
        + (date1.month * 306 as std::os::raw::c_int + 5 as std::os::raw::c_int) / 10 as std::os::raw::c_int
        + (date1.day - 1 as std::os::raw::c_int);
    days2 = 365 as std::os::raw::c_int * date2.year + date2.year / 4 as std::os::raw::c_int
        - date2.year / 100 as std::os::raw::c_int + date2.year / 400 as std::os::raw::c_int
        + (date2.month * 306 as std::os::raw::c_int + 5 as std::os::raw::c_int) / 10 as std::os::raw::c_int
        + (date2.day - 1 as std::os::raw::c_int);
    return days2 - days1;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    if argc != 3 as std::os::raw::c_int {
        return printf(
            b"Usage : %s <yyyy-mm-dd> <yyyy-mm-dd>\0" as *const u8
                as *const std::os::raw::c_char,
            *argv.offset(0 as std::os::raw::c_int as isize),
        );
    }
    if isValidDate(*argv.offset(1 as std::os::raw::c_int as isize)) as std::os::raw::c_int != 0
        && isValidDate(*argv.offset(2 as std::os::raw::c_int as isize)) as std::os::raw::c_int
            == 0 as std::os::raw::c_int
    {
        return printf(b"Dates are invalid.\n\0" as *const u8 as *const std::os::raw::c_char);
    }
    printf(
        b"Days Difference : %d\n\0" as *const u8 as *const std::os::raw::c_char,
        diffDays(
            extractDate(*argv.offset(1 as std::os::raw::c_int as isize)),
            extractDate(*argv.offset(2 as std::os::raw::c_int as isize)),
        ),
    );
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

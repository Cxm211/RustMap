#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut days: [std::os::raw::c_int; 12] = [
        31 as std::os::raw::c_int,
        29 as std::os::raw::c_int,
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
    let mut m: std::os::raw::c_int = 0;
    let mut y: std::os::raw::c_int = 0;
    let mut w: std::os::raw::c_int = 0;
    if argc < 2 as std::os::raw::c_int
        || {
            y = atoi(*argv.offset(1 as std::os::raw::c_int as isize));
            y <= 1752 as std::os::raw::c_int
        }
    {
        return 1 as std::os::raw::c_int;
    }
    days[1 as std::os::raw::c_int as usize]
        -= (y % 4 as std::os::raw::c_int != 0
            || y % 100 as std::os::raw::c_int == 0 && y % 400 as std::os::raw::c_int != 0)
            as std::os::raw::c_int;
    w = y * 365 as std::os::raw::c_int
        + 97 as std::os::raw::c_int * (y - 1 as std::os::raw::c_int) / 400 as std::os::raw::c_int
        + 4 as std::os::raw::c_int;
    m = 0 as std::os::raw::c_int;
    while m < 12 as std::os::raw::c_int {
        w = (w + days[m as usize]) % 7 as std::os::raw::c_int;
        printf(
            b"%d-%02d-%d\n\0" as *const u8 as *const std::os::raw::c_char,
            y,
            m + 1 as std::os::raw::c_int,
            days[m as usize] - w,
        );
        m += 1;
        m;
    }
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

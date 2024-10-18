#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn wday(
    mut year: std::os::raw::c_int,
    mut month: std::os::raw::c_int,
    mut day: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut adjustment: std::os::raw::c_int = 0;
    let mut mm: std::os::raw::c_int = 0;
    let mut yy: std::os::raw::c_int = 0;
    adjustment = (14 as std::os::raw::c_int - month) / 12 as std::os::raw::c_int;
    mm = month + 12 as std::os::raw::c_int * adjustment - 2 as std::os::raw::c_int;
    yy = year - adjustment;
    return (day + (13 as std::os::raw::c_int * mm - 1 as std::os::raw::c_int) / 5 as std::os::raw::c_int + yy
        + yy / 4 as std::os::raw::c_int - yy / 100 as std::os::raw::c_int + yy / 400 as std::os::raw::c_int)
        % 7 as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut y: std::os::raw::c_int = 0;
    y = 2008 as std::os::raw::c_int;
    while y <= 2121 as std::os::raw::c_int {
        if wday(y, 12 as std::os::raw::c_int, 25 as std::os::raw::c_int) == 0 as std::os::raw::c_int {
            printf(b"%04d-12-25\n\0" as *const u8 as *const std::os::raw::c_char, y);
        }
        y += 1;
        y;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

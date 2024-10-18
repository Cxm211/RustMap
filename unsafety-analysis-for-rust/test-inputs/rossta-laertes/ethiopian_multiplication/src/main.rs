#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn halve<'a1>(mut x: Option<&'a1 mut std::os::raw::c_int>) {
    *borrow_mut(&mut x).unwrap() >>= 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn doublit<'a1>(mut x: Option<&'a1 mut std::os::raw::c_int>) {
    *borrow_mut(&mut x).unwrap() <<= 1 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn iseven(x: std::os::raw::c_int) -> bool {
    return x & 1 as std::os::raw::c_int == 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ethiopian(
    mut plier: std::os::raw::c_int,
    mut plicand: std::os::raw::c_int,
    tutor: bool,
) -> std::os::raw::c_int {
    let mut result: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if tutor {
        printf(
            b"ethiopian multiplication of %d by %d\n\0" as *const u8
                as *const std::os::raw::c_char,
            plier,
            plicand,
        );
    }
    while plier >= 1 as std::os::raw::c_int {
        if iseven(plier) {
            if tutor {
                printf(
                    b"%4d %6d struck\n\0" as *const u8 as *const std::os::raw::c_char,
                    plier,
                    plicand,
                );
            }
        } else {
            if tutor {
                printf(
                    b"%4d %6d kept\n\0" as *const u8 as *const std::os::raw::c_char,
                    plier,
                    plicand,
                );
            }
            result += plicand;
        }
        halve(Some(&mut plier));
        doublit(Some(&mut plicand));
    }
    return result;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        ethiopian(17 as std::os::raw::c_int, 34 as std::os::raw::c_int, 1 as std::os::raw::c_int != 0),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

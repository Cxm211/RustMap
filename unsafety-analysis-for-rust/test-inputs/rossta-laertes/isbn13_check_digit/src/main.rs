#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check_isbn13(mut isbn: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut ch: std::os::raw::c_int = *isbn as std::os::raw::c_int;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while ch != 0 as std::os::raw::c_int {
        if ch == ' ' as i32 || ch == '-' as i32 {
            count -= 1;
            count;
        } else {
            if ch < '0' as i32 || ch > '9' as i32 {
                return 0 as std::os::raw::c_int;
            }
            if count & 1 as std::os::raw::c_int != 0 {
                sum += 3 as std::os::raw::c_int * (ch - '0' as i32);
            } else {
                sum += ch - '0' as i32;
            }
        }
        isbn = isbn.offset(1);
        ch = *isbn as std::os::raw::c_int;
        count += 1;
        count;
    }
    if count != 13 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    return (sum % 10 as std::os::raw::c_int == 0) as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut isbns: [*const std::os::raw::c_char; 4] = [
        b"978-1734314502\0" as *const u8 as *const std::os::raw::c_char,
        b"978-1734314509\0" as *const u8 as *const std::os::raw::c_char,
        b"978-1788399081\0" as *const u8 as *const std::os::raw::c_char,
        b"978-1788399083\0" as *const u8 as *const std::os::raw::c_char,
    ];
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        printf(
            b"%s: %s\n\0" as *const u8 as *const std::os::raw::c_char,
            isbns[i as usize],
            if check_isbn13(isbns[i as usize]) != 0 {
                b"good\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"bad\0" as *const u8 as *const std::os::raw::c_char
            },
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

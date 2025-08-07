#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 12 as std::os::raw::c_int;
    j = 1 as std::os::raw::c_int;
    while j <= n {
        printf(
            b"%3d%c\0" as *const u8 as *const std::os::raw::c_char,
            j,
            if j != n { ' ' as i32 } else { '\n' as i32 },
        );
        j += 1;
        j;
    }
    j = 0 as std::os::raw::c_int;
    while j <= n {
        printf(
            if j != n {
                b"----\0" as *const u8 as *const std::os::raw::c_char
            } else {
                b"+\n\0" as *const u8 as *const std::os::raw::c_char
            },
        );
        j += 1;
        j;
    }
    i = 1 as std::os::raw::c_int;
    while i <= n {
        j = 1 as std::os::raw::c_int;
        while j <= n {
            printf(
                if j < i {
                    b"    \0" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"%3d \0" as *const u8 as *const std::os::raw::c_char
                },
                i * j,
            );
            j += 1;
            j;
        }
        printf(b"| %d\n\0" as *const u8 as *const std::os::raw::c_char, i);
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

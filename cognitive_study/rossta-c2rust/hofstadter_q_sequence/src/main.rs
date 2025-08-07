#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut flip: std::os::raw::c_int = 0;
    let mut q: *mut std::os::raw::c_int = (malloc(
        (::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(100000 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int)
        .offset(-(1 as std::os::raw::c_int as isize));
    let ref mut fresh0 = *q.offset(2 as std::os::raw::c_int as isize);
    *fresh0 = 1 as std::os::raw::c_int;
    *q.offset(1 as std::os::raw::c_int as isize) = *fresh0;
    i = 3 as std::os::raw::c_int;
    while i <= 100000 as std::os::raw::c_int {
        *q
            .offset(
                i as isize,
            ) = *q.offset((i - *q.offset((i - 1 as std::os::raw::c_int) as isize)) as isize)
            + *q.offset((i - *q.offset((i - 2 as std::os::raw::c_int) as isize)) as isize);
        i += 1;
        i;
    }
    i = 1 as std::os::raw::c_int;
    while i <= 10 as std::os::raw::c_int {
        printf(
            b"%d%c\0" as *const u8 as *const std::os::raw::c_char,
            *q.offset(i as isize),
            if i == 10 as std::os::raw::c_int { '\n' as i32 } else { ' ' as i32 },
        );
        i += 1;
        i;
    }
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        *q.offset(1000 as std::os::raw::c_int as isize),
    );
    flip = 0 as std::os::raw::c_int;
    i = 1 as std::os::raw::c_int;
    while i < 100000 as std::os::raw::c_int {
        flip
            += (*q.offset(i as isize) > *q.offset((i + 1 as std::os::raw::c_int) as isize))
                as std::os::raw::c_int;
        i += 1;
        i;
    }
    printf(b"flips: %d\n\0" as *const u8 as *const std::os::raw::c_char, flip);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

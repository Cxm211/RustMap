#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn snprintf(
        _: *mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn rprint(
    mut s: *mut std::os::raw::c_char,
    mut x: *mut std::os::raw::c_int,
    mut len: std::os::raw::c_int,
) -> size_t {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut a: *mut std::os::raw::c_char = s;
    j = 0 as std::os::raw::c_int;
    i = j;
    while i < len {
        while j < len - 1 as std::os::raw::c_int
            && *x.offset((j + 1 as std::os::raw::c_int) as isize)
                == *x.offset(j as isize) + 1 as std::os::raw::c_int
        {
            j += 1;
            j;
        }
        if (i + 1 as std::os::raw::c_int) < j {
            a = a
                .offset(
                    snprintf(
                        if !s.is_null() { a } else { s },
                        (if !s.is_null() {
                            100 as std::os::raw::c_int
                        } else {
                            0 as std::os::raw::c_int
                        }) as std::os::raw::c_ulong,
                        b"%s%d-%d\0" as *const u8 as *const std::os::raw::c_char,
                        if a > s {
                            b",\0" as *const u8 as *const std::os::raw::c_char
                        } else {
                            b"\0" as *const u8 as *const std::os::raw::c_char
                        },
                        *x.offset(i as isize),
                        *x.offset(j as isize),
                    ) as isize,
                );
        } else {
            while i <= j {
                let fresh0 = i;
                i = i + 1;
                a = a
                    .offset(
                        snprintf(
                            if !s.is_null() { a } else { s },
                            (if !s.is_null() {
                                100 as std::os::raw::c_int
                            } else {
                                0 as std::os::raw::c_int
                            }) as std::os::raw::c_ulong,
                            b"%s%d\0" as *const u8 as *const std::os::raw::c_char,
                            if a > s {
                                b",\0" as *const u8 as *const std::os::raw::c_char
                            } else {
                                b"\0" as *const u8 as *const std::os::raw::c_char
                            },
                            *x.offset(fresh0 as isize),
                        ) as isize,
                    );
            }
        }
        j += 1;
        i = j;
    }
    return a.offset_from(s) as std::os::raw::c_long as size_t;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: [std::os::raw::c_int; 33] = [
        0 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        6 as std::os::raw::c_int,
        7 as std::os::raw::c_int,
        8 as std::os::raw::c_int,
        11 as std::os::raw::c_int,
        12 as std::os::raw::c_int,
        14 as std::os::raw::c_int,
        15 as std::os::raw::c_int,
        16 as std::os::raw::c_int,
        17 as std::os::raw::c_int,
        18 as std::os::raw::c_int,
        19 as std::os::raw::c_int,
        20 as std::os::raw::c_int,
        21 as std::os::raw::c_int,
        22 as std::os::raw::c_int,
        23 as std::os::raw::c_int,
        24 as std::os::raw::c_int,
        25 as std::os::raw::c_int,
        27 as std::os::raw::c_int,
        28 as std::os::raw::c_int,
        29 as std::os::raw::c_int,
        30 as std::os::raw::c_int,
        31 as std::os::raw::c_int,
        32 as std::os::raw::c_int,
        33 as std::os::raw::c_int,
        35 as std::os::raw::c_int,
        36 as std::os::raw::c_int,
        37 as std::os::raw::c_int,
        38 as std::os::raw::c_int,
        39 as std::os::raw::c_int,
    ];
    let mut s: *mut std::os::raw::c_char = malloc(
        (rprint(
            0 as *mut std::os::raw::c_char,
            x.as_mut_ptr(),
            (std::mem::size_of::<[std::os::raw::c_int; 33]>() as std::os::raw::c_ulong)
                .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
                as std::os::raw::c_int,
        ))
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    rprint(
        s,
        x.as_mut_ptr(),
        (std::mem::size_of::<[std::os::raw::c_int; 33]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
    );
    printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, s);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

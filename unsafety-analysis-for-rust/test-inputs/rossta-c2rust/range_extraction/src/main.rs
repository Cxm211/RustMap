#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn rprint(
    mut s: *mut libc::c_char,
    mut x: *mut libc::c_int,
    mut len: libc::c_int,
) -> size_t {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut a: *mut libc::c_char = s;
    j = 0 as libc::c_int;
    i = j;
    while i < len {
        while j < len - 1 as libc::c_int
            && *x.offset((j + 1 as libc::c_int) as isize)
                == *x.offset(j as isize) + 1 as libc::c_int
        {
            j += 1;
            j;
        }
        if (i + 1 as libc::c_int) < j {
            a = a
                .offset(
                    snprintf(
                        if !s.is_null() { a } else { s },
                        (if !s.is_null() {
                            100 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as libc::c_ulong,
                        b"%s%d-%d\0" as *const u8 as *const libc::c_char,
                        if a > s {
                            b",\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
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
                                100 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) as libc::c_ulong,
                            b"%s%d\0" as *const u8 as *const libc::c_char,
                            if a > s {
                                b",\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            *x.offset(fresh0 as isize),
                        ) as isize,
                    );
            }
        }
        j += 1;
        i = j;
    }
    return a.offset_from(s) as libc::c_long as size_t;
}
unsafe fn main_0() -> libc::c_int {
    let mut x: [libc::c_int; 33] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        11 as libc::c_int,
        12 as libc::c_int,
        14 as libc::c_int,
        15 as libc::c_int,
        16 as libc::c_int,
        17 as libc::c_int,
        18 as libc::c_int,
        19 as libc::c_int,
        20 as libc::c_int,
        21 as libc::c_int,
        22 as libc::c_int,
        23 as libc::c_int,
        24 as libc::c_int,
        25 as libc::c_int,
        27 as libc::c_int,
        28 as libc::c_int,
        29 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        32 as libc::c_int,
        33 as libc::c_int,
        35 as libc::c_int,
        36 as libc::c_int,
        37 as libc::c_int,
        38 as libc::c_int,
        39 as libc::c_int,
    ];
    let mut s: *mut libc::c_char = malloc(
        (rprint(
            0 as *mut libc::c_char,
            x.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_int; 33]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
        ))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    rprint(
        s,
        x.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_int; 33]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    );
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, s);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type xint = std::os::raw::c_ulong;
#[no_mangle]
pub static mut total: xint = 0;
#[no_mangle]
pub static mut prim: xint = 0;
#[no_mangle]
pub static mut max_peri: xint = 0;
#[no_mangle]
pub static mut U: [[xint; 9]; 3] = [
    [
        1 as std::os::raw::c_int as xint,
        -(2 as std::os::raw::c_int) as xint,
        2 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        -(1 as std::os::raw::c_int) as xint,
        2 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        -(2 as std::os::raw::c_int) as xint,
        3 as std::os::raw::c_int as xint,
    ],
    [
        1 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        1 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        3 as std::os::raw::c_int as xint,
    ],
    [
        -(1 as std::os::raw::c_int) as xint,
        2 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        -(2 as std::os::raw::c_int) as xint,
        1 as std::os::raw::c_int as xint,
        2 as std::os::raw::c_int as xint,
        -(2 as std::os::raw::c_int) as xint,
        2 as std::os::raw::c_int as xint,
        3 as std::os::raw::c_int as xint,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn new_tri(mut in_0: *mut xint) {
    let mut i: std::os::raw::c_int = 0;
    let mut t: [xint; 3] = [0; 3];
    let mut p: xint = (*in_0.offset(0 as std::os::raw::c_int as isize))
        .wrapping_add(*in_0.offset(1 as std::os::raw::c_int as isize))
        .wrapping_add(*in_0.offset(2 as std::os::raw::c_int as isize));
    if p > max_peri {
        return;
    }
    prim = prim.wrapping_add(1);
    prim;
    total = (total as std::os::raw::c_ulong).wrapping_add(max_peri.wrapping_div(p)) as xint
        as xint;
    i = 0 as std::os::raw::c_int;
    while i < 3 as std::os::raw::c_int {
        t[0 as std::os::raw::c_int
            as usize] = (U[i as usize][0 as std::os::raw::c_int as usize])
            .wrapping_mul(*in_0.offset(0 as std::os::raw::c_int as isize))
            .wrapping_add(
                (U[i as usize][1 as std::os::raw::c_int as usize])
                    .wrapping_mul(*in_0.offset(1 as std::os::raw::c_int as isize)),
            )
            .wrapping_add(
                (U[i as usize][2 as std::os::raw::c_int as usize])
                    .wrapping_mul(*in_0.offset(2 as std::os::raw::c_int as isize)),
            );
        t[1 as std::os::raw::c_int
            as usize] = (U[i as usize][3 as std::os::raw::c_int as usize])
            .wrapping_mul(*in_0.offset(0 as std::os::raw::c_int as isize))
            .wrapping_add(
                (U[i as usize][4 as std::os::raw::c_int as usize])
                    .wrapping_mul(*in_0.offset(1 as std::os::raw::c_int as isize)),
            )
            .wrapping_add(
                (U[i as usize][5 as std::os::raw::c_int as usize])
                    .wrapping_mul(*in_0.offset(2 as std::os::raw::c_int as isize)),
            );
        t[2 as std::os::raw::c_int
            as usize] = (U[i as usize][6 as std::os::raw::c_int as usize])
            .wrapping_mul(*in_0.offset(0 as std::os::raw::c_int as isize))
            .wrapping_add(
                (U[i as usize][7 as std::os::raw::c_int as usize])
                    .wrapping_mul(*in_0.offset(1 as std::os::raw::c_int as isize)),
            )
            .wrapping_add(
                (U[i as usize][8 as std::os::raw::c_int as usize])
                    .wrapping_mul(*in_0.offset(2 as std::os::raw::c_int as isize)),
            );
        new_tri(t.as_mut_ptr());
        i += 1;
        i;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut seed: [xint; 3] = [
        3 as std::os::raw::c_int as xint,
        4 as std::os::raw::c_int as xint,
        5 as std::os::raw::c_int as xint,
    ];
    max_peri = 10 as std::os::raw::c_int as xint;
    while max_peri <= 100000000 as std::os::raw::c_int as std::os::raw::c_ulong {
        prim = 0 as std::os::raw::c_int as xint;
        total = prim;
        new_tri(seed.as_mut_ptr());
        printf(
            b"Up to %lu: %lu triples, %lu primitives.\n\0" as *const u8
                as *const std::os::raw::c_char,
            max_peri,
            total,
            prim,
        );
        max_peri = (max_peri as std::os::raw::c_ulong)
            .wrapping_mul(10 as std::os::raw::c_int as std::os::raw::c_ulong) as xint as xint;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

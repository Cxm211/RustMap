#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn anynacci(
    mut seedArray: *mut std::os::raw::c_int,
    mut howMany: std::os::raw::c_int,
) -> *mut std::os::raw::c_int {
    let mut result: *mut std::os::raw::c_int = malloc(
        (howMany as std::os::raw::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut initialCardinality: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while *seedArray.offset(i as isize) != 0 as std::os::raw::c_int {
        i += 1;
        i;
    }
    initialCardinality = i;
    i = 0 as std::os::raw::c_int;
    while i < initialCardinality {
        *result.offset(i as isize) = *seedArray.offset(i as isize);
        i += 1;
        i;
    }
    i = initialCardinality;
    while i < howMany {
        *result.offset(i as isize) = 0 as std::os::raw::c_int;
        j = i - initialCardinality;
        while j < i {
            *result.offset(i as isize) += *result.offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return result;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut fibo: [std::os::raw::c_int; 3] = [
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ];
    let mut tribo: [std::os::raw::c_int; 4] = [
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ];
    let mut tetra: [std::os::raw::c_int; 5] = [
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ];
    let mut luca: [std::os::raw::c_int; 3] = [
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ];
    let mut fibonacci: *mut std::os::raw::c_int = anynacci(fibo.as_mut_ptr(), 10 as std::os::raw::c_int);
    let mut tribonacci: *mut std::os::raw::c_int = anynacci(
        tribo.as_mut_ptr(),
        10 as std::os::raw::c_int,
    );
    let mut tetranacci: *mut std::os::raw::c_int = anynacci(
        tetra.as_mut_ptr(),
        10 as std::os::raw::c_int,
    );
    let mut lucas: *mut std::os::raw::c_int = anynacci(luca.as_mut_ptr(), 10 as std::os::raw::c_int);
    let mut i: std::os::raw::c_int = 0;
    printf(
        b"\nFibonacci\tTribonacci\tTetranacci\tLucas\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        printf(
            b"\n%d\t\t%d\t\t%d\t\t%d\0" as *const u8 as *const std::os::raw::c_char,
            *fibonacci.offset(i as isize),
            *tribonacci.offset(i as isize),
            *tetranacci.offset(i as isize),
            *lucas.offset(i as isize),
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

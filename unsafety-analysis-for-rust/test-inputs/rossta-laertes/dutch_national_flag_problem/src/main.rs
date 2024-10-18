#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn qsort(
        __base: *mut u8,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = std::os::raw::c_ulong;
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
pub type __compar_fn_t = Option<unsafe extern "C"  fn(_: * const u8,_: * const u8,) -> std::os::raw::c_int>;
#[no_mangle]
pub unsafe extern "C" fn compar(
    mut a: *const u8,
    mut b: *const u8,
) -> std::os::raw::c_int {
    let mut c1: std::os::raw::c_char = *(a as *const std::os::raw::c_char);
    let mut c2: std::os::raw::c_char = *(b as *const std::os::raw::c_char);
    return c1 as std::os::raw::c_int - c2 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn issorted(mut balls: *mut std::os::raw::c_char) -> bool {
    let mut i: std::os::raw::c_int = 0;
    let mut state: std::os::raw::c_int = 0;
    state = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        if (*balls.offset(i as isize) as std::os::raw::c_int) < state {
            return 0 as std::os::raw::c_int != 0;
        }
        if *balls.offset(i as isize) as std::os::raw::c_int > state {
            state = *balls.offset(i as isize) as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn printout(mut balls: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    let mut str: [std::os::raw::c_char; 6] = [0; 6];
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        str[i
            as usize] = (if *balls.offset(i as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
        {
            'r' as i32
        } else if *balls.offset(i as isize) as std::os::raw::c_int == 1 as std::os::raw::c_int {
            'w' as i32
        } else {
            'b' as i32
        }) as std::os::raw::c_char;
        i += 1;
        i;
    }
    printf(b"%s\n\0" as *const u8 as *const std::os::raw::c_char, str.as_mut_ptr());
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut balls: [std::os::raw::c_char; 5] = [0; 5];
    let mut i: std::os::raw::c_int = 0;
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    rand();
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        balls[i
            as usize] = (rand() as std::os::raw::c_double
            / 2147483647 as std::os::raw::c_int as std::os::raw::c_double
            * 3 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_char;
        i += 1;
        i;
    }
    while issorted(balls.as_mut_ptr()) {
        printf(b"Accidentally still sorted: \0" as *const u8 as *const std::os::raw::c_char);
        printout(balls.as_mut_ptr());
        i = 0 as std::os::raw::c_int;
        while i < 5 as std::os::raw::c_int {
            balls[i
                as usize] = (rand() as std::os::raw::c_double
                / 2147483647 as std::os::raw::c_int as std::os::raw::c_double
                * 3 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_char;
            i += 1;
            i;
        }
    }
    printf(b"Non-sorted: \0" as *const u8 as *const std::os::raw::c_char);
    printout(balls.as_mut_ptr());
    qsort(
        balls.as_mut_ptr() as *mut u8,
        5 as std::os::raw::c_int as size_t,
        std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong,
        Some(
            compar
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    if issorted(balls.as_mut_ptr()) {
        printf(b"Sorted: \0" as *const u8 as *const std::os::raw::c_char);
        printout(balls.as_mut_ptr());
    } else {
        printf(b"Sort failed: \0" as *const u8 as *const std::os::raw::c_char);
        printout(balls.as_mut_ptr());
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

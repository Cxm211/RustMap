#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}
#[no_mangle]
pub static mut target: [std::os::raw::c_char; 29] = unsafe {
    *std::mem::transmute::<
        &[u8; 29],
        &[std::os::raw::c_char; 29],
    >(b"METHINKS IT IS LIKE A WEASEL\0")
};
#[no_mangle]
pub static mut tbl: [std::os::raw::c_char; 28] = unsafe {
    *std::mem::transmute::<
        &[u8; 28],
        &[std::os::raw::c_char; 28],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ \0")
};
#[no_mangle]
pub unsafe extern "C" fn irand(mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut r: std::os::raw::c_int = 0;
    let mut rand_max: std::os::raw::c_int = 2147483647 as std::os::raw::c_int
        - 2147483647 as std::os::raw::c_int % n;
    loop {
        r = rand();
        if !(r >= rand_max) {
            break;
        }
    }
    return r / (rand_max / n);
}
#[no_mangle]
pub unsafe extern "C" fn unfitness(
    mut a: *const std::os::raw::c_char,
    mut b: *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while *a.offset(i as isize) != 0 {
        sum
            += (*a.offset(i as isize) as std::os::raw::c_int
                != *b.offset(i as isize) as std::os::raw::c_int) as std::os::raw::c_int;
        i += 1;
        i;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn mutate(mut a: *const std::os::raw::c_char, mut b: *mut std::os::raw::c_char) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while *a.offset(i as isize) != 0 {
        *b
            .offset(
                i as isize,
            ) = (if irand(15 as std::os::raw::c_int) != 0 {
            *a.offset(i as isize) as std::os::raw::c_int
        } else {
            tbl[irand(
                (std::mem::size_of::<[std::os::raw::c_char; 28]>() as std::os::raw::c_ulong)
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int,
            ) as usize] as std::os::raw::c_int
        }) as std::os::raw::c_char;
        i += 1;
        i;
    }
    *b.offset(i as isize) = '\0' as i32 as std::os::raw::c_char;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut best_i: std::os::raw::c_int = 0;
    let mut unfit: std::os::raw::c_int = 0;
    let mut best: std::os::raw::c_int = 0;
    let mut iters: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut specimen: [[std::os::raw::c_char; 29]; 30] = [[0; 29]; 30];
    i = 0 as std::os::raw::c_int;
    while target[i as usize] != 0 {
        specimen[0 as std::os::raw::c_int
            as usize][i
            as usize] = tbl[irand(
            (std::mem::size_of::<[std::os::raw::c_char; 28]>() as std::os::raw::c_ulong)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int,
        ) as usize];
        i += 1;
        i;
    }
    specimen[0 as std::os::raw::c_int as usize][i as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    loop {
        i = 1 as std::os::raw::c_int;
        while i < 30 as std::os::raw::c_int {
            mutate(
                (specimen[0 as std::os::raw::c_int as usize]).as_mut_ptr(),
                (specimen[i as usize]).as_mut_ptr(),
            );
            i += 1;
            i;
        }
        i = 0 as std::os::raw::c_int;
        best_i = i;
        while i < 30 as std::os::raw::c_int {
            unfit = unfitness(target.as_ptr(), (specimen[i as usize]).as_mut_ptr());
            if unfit < best || i == 0 {
                best = unfit;
                best_i = i;
            }
            i += 1;
            i;
        }
        if best_i != 0 {
            strcpy(
                (specimen[0 as std::os::raw::c_int as usize]).as_mut_ptr(),
                (specimen[best_i as usize]).as_mut_ptr(),
            );
        }
        let fresh0 = iters;
        iters = iters + 1;
        printf(
            b"iter %d, score %d: %s\n\0" as *const u8 as *const std::os::raw::c_char,
            fresh0,
            best,
            (specimen[0 as std::os::raw::c_int as usize]).as_mut_ptr(),
        );
        if !(best != 0) {
            break;
        }
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

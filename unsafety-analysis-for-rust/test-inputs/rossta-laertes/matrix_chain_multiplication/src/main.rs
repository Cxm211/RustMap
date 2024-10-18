#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
#[no_mangle]
pub static mut m: *mut *mut std::os::raw::c_int = 0 as *const *mut std::os::raw::c_int
    as *mut *mut std::os::raw::c_int;
#[no_mangle]
pub static mut s: *mut *mut std::os::raw::c_int = 0 as *const *mut std::os::raw::c_int
    as *mut *mut std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn optimal_matrix_chain_order(
    mut dims: *mut std::os::raw::c_int,
    mut n: std::os::raw::c_int,
) {
    let mut len: std::os::raw::c_int = 0;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut temp: std::os::raw::c_int = 0;
    let mut cost: std::os::raw::c_int = 0;
    n -= 1;
    n;
    m = malloc(
        (n as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < n {
        let ref mut fresh0 = *m.offset(i as isize);
        *fresh0 = calloc(
            n as std::os::raw::c_ulong,
            std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_int;
        i += 1;
        i;
    }
    s = malloc(
        (n as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<*mut std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut *mut std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < n {
        let ref mut fresh1 = *s.offset(i as isize);
        *fresh1 = calloc(
            n as std::os::raw::c_ulong,
            std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
        ) as *mut std::os::raw::c_int;
        i += 1;
        i;
    }
    len = 1 as std::os::raw::c_int;
    while len < n {
        i = 0 as std::os::raw::c_int;
        while i < n - len {
            j = i + len;
            *(*m.offset(i as isize)).offset(j as isize) = 2147483647 as std::os::raw::c_int;
            k = i;
            while k < j {
                temp = *dims.offset(i as isize)
                    * *dims.offset((k + 1 as std::os::raw::c_int) as isize)
                    * *dims.offset((j + 1 as std::os::raw::c_int) as isize);
                cost = *(*m.offset(i as isize)).offset(k as isize)
                    + *(*m.offset((k + 1 as std::os::raw::c_int) as isize)).offset(j as isize)
                    + temp;
                if cost < *(*m.offset(i as isize)).offset(j as isize) {
                    *(*m.offset(i as isize)).offset(j as isize) = cost;
                    *(*s.offset(i as isize)).offset(j as isize) = k;
                }
                k += 1;
                k;
            }
            i += 1;
            i;
        }
        len += 1;
        len;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_optimal_chain_order(
    mut i: std::os::raw::c_int,
    mut j: std::os::raw::c_int,
) {
    if i == j {
        printf(b"%c\0" as *const u8 as *const std::os::raw::c_char, i + 65 as std::os::raw::c_int);
    } else {
        printf(b"(\0" as *const u8 as *const std::os::raw::c_char);
        print_optimal_chain_order(i, *(*s.offset(i as isize)).offset(j as isize));
        print_optimal_chain_order(
            *(*s.offset(i as isize)).offset(j as isize) + 1 as std::os::raw::c_int,
            j,
        );
        printf(b")\0" as *const u8 as *const std::os::raw::c_char);
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    let mut a1: [std::os::raw::c_int; 4] = [
        5 as std::os::raw::c_int,
        6 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    ];
    let mut a2: [std::os::raw::c_int; 13] = [
        1 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        25 as std::os::raw::c_int,
        30 as std::os::raw::c_int,
        100 as std::os::raw::c_int,
        70 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        100 as std::os::raw::c_int,
        250 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        1000 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
    ];
    let mut a3: [std::os::raw::c_int; 12] = [
        1000 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        500 as std::os::raw::c_int,
        12 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        700 as std::os::raw::c_int,
        2500 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        5 as std::os::raw::c_int,
        14 as std::os::raw::c_int,
        10 as std::os::raw::c_int,
    ];
    let mut dims_list: [*mut std::os::raw::c_int; 3] = [
        a1.as_mut_ptr(),
        a2.as_mut_ptr(),
        a3.as_mut_ptr(),
    ];
    let mut sizes: [std::os::raw::c_int; 3] = [
        4 as std::os::raw::c_int,
        13 as std::os::raw::c_int,
        12 as std::os::raw::c_int,
    ];
    i = 0 as std::os::raw::c_int;
    while i < 3 as std::os::raw::c_int {
        printf(b"Dims  : [\0" as *const u8 as *const std::os::raw::c_char);
        n = sizes[i as usize];
        j = 0 as std::os::raw::c_int;
        while j < n {
            printf(
                b"%d\0" as *const u8 as *const std::os::raw::c_char,
                *(dims_list[i as usize]).offset(j as isize),
            );
            if j < n - 1 as std::os::raw::c_int {
                printf(b", \0" as *const u8 as *const std::os::raw::c_char);
            } else {
                printf(b"]\n\0" as *const u8 as *const std::os::raw::c_char);
            }
            j += 1;
            j;
        }
        optimal_matrix_chain_order(dims_list[i as usize], n);
        printf(b"Order : \0" as *const u8 as *const std::os::raw::c_char);
        print_optimal_chain_order(0 as std::os::raw::c_int, n - 2 as std::os::raw::c_int);
        printf(
            b"\nCost  : %d\n\n\0" as *const u8 as *const std::os::raw::c_char,
            *(*m.offset(0 as std::os::raw::c_int as isize))
                .offset((n - 2 as std::os::raw::c_int) as isize),
        );
        j = 0 as std::os::raw::c_int;
        while j <= n - 2 as std::os::raw::c_int {
            free(*m.offset(j as isize) as *mut u8);
            j += 1;
            j;
        }
        free(m as *mut u8);
        j = 0 as std::os::raw::c_int;
        while j <= n - 2 as std::os::raw::c_int {
            free(*s.offset(j as isize) as *mut u8);
            j += 1;
            j;
        }
        free(s as *mut u8);
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

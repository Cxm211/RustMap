#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub extern "C" fn bellIndex(
    mut row: std::os::raw::c_int,
    mut col: std::os::raw::c_int,
) -> size_t {
    return (row * (row - 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int + col) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn getBell(
    mut bellTri: *mut std::os::raw::c_int,
    mut row: std::os::raw::c_int,
    mut col: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut index: size_t = bellIndex(row, col);
    return *bellTri.offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn setBell(
    mut bellTri: *mut std::os::raw::c_int,
    mut row: std::os::raw::c_int,
    mut col: std::os::raw::c_int,
    mut value: std::os::raw::c_int,
) {
    let mut index: size_t = bellIndex(row, col);
    *bellTri.offset(index as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn bellTriangle(mut n: std::os::raw::c_int) -> *mut std::os::raw::c_int {
    let mut length: size_t = (n * (n + 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int) as size_t;
    let mut tri: *mut std::os::raw::c_int = calloc(
        length,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    setBell(tri, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int, 1 as std::os::raw::c_int);
    i = 2 as std::os::raw::c_int;
    while i <= n {
        setBell(
            tri,
            i,
            0 as std::os::raw::c_int,
            getBell(tri, i - 1 as std::os::raw::c_int, i - 2 as std::os::raw::c_int),
        );
        j = 1 as std::os::raw::c_int;
        while j < i {
            let mut value: std::os::raw::c_int = getBell(tri, i, j - 1 as std::os::raw::c_int)
                + getBell(tri, i - 1 as std::os::raw::c_int, j - 1 as std::os::raw::c_int);
            setBell(tri, i, j, value);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return tri;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let rows: std::os::raw::c_int = 15 as std::os::raw::c_int;
    let mut bt: *mut std::os::raw::c_int = bellTriangle(rows);
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    printf(b"First fifteen Bell numbers:\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 1 as std::os::raw::c_int;
    while i <= rows {
        printf(
            b"%2d: %d\n\0" as *const u8 as *const std::os::raw::c_char,
            i,
            getBell(bt, i, 0 as std::os::raw::c_int),
        );
        i += 1;
        i;
    }
    printf(
        b"\nThe first ten rows of Bell's triangle:\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    i = 1 as std::os::raw::c_int;
    while i <= 10 as std::os::raw::c_int {
        printf(
            b"%d\0" as *const u8 as *const std::os::raw::c_char,
            getBell(bt, i, 0 as std::os::raw::c_int),
        );
        j = 1 as std::os::raw::c_int;
        while j < i {
            printf(b", %d\0" as *const u8 as *const std::os::raw::c_char, getBell(bt, i, j));
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    free(bt as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn bellIndex(mut row: i32, mut col: i32) -> u64 {
    return (row * (row - 1i32) / 2 + col) as u64;
}

#[no_mangle]
pub extern "C" fn getBell(mut bellTri: *mut i32, mut row: i32, mut col: i32) -> i32 {
    unsafe {
        let mut index: u64 = bellIndex(row, col);
        return *bellTri.offset(index as isize);
    }
}

#[no_mangle]
pub extern "C" fn setBell(mut bellTri: *mut i32, mut row: i32, mut col: i32, mut value: i32) {
    unsafe {
        let mut index: u64 = bellIndex(row, col);
        *bellTri.offset(index as isize) = value;
    }
}

#[no_mangle]
pub extern "C" fn bellTriangle(mut n: i32) -> *mut i32 {
    unsafe {
        let mut length: u64 = (n * (n + 1i32) / 2) as u64;
        let mut tri: *mut i32 = calloc(length, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        setBell(tri, 1, 0, 1);
        i = 2;
        while i <= n {
            setBell(tri, i, 0, getBell(tri, i - 1, i - 2));
            j = 1;
            while j < i {
                let mut value: i32 = getBell(tri, i, j - 1) + getBell(tri, i - 1, j - 1);
                setBell(tri, i, j, value);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        return tri;
    }
}

fn main_0() -> i32 {
    unsafe {
        let rows: i32 = 15;
        let mut bt: *mut i32 = bellTriangle(rows);
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        print!("First fifteen Bell numbers:\n");
        i = 1;
        while i <= rows {
            print!("{:2}: {}\n", i, getBell(bt, i, 0));
            i += 1;
            i;
        }
        printf(b"\nThe first ten rows of Bell's triangle:\n\0" as *const u8 as *const i8);
        i = 1;
        while i <= 10 {
            print!("{}", getBell(bt, i, 0));
            j = 1;
            while j < i {
                print!(", {}", getBell(bt, i, j));
                j += 1;
                j;
            }
            print!("\n");
            i += 1;
            i;
        }
        free(bt as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

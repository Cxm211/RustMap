#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn realloc(_: *mut u8, _: std::os::raw::c_ulong) -> *mut u8;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
}
pub type byte = std::os::raw::c_uchar;
#[no_mangle]
pub static mut grid: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut w: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut h: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut len: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut cnt: std::os::raw::c_ulonglong = 0;
static mut next: [std::os::raw::c_int; 4] = [0; 4];
static mut dir: [[std::os::raw::c_int; 2]; 4] = [
    [0 as std::os::raw::c_int, -(1 as std::os::raw::c_int)],
    [-(1 as std::os::raw::c_int), 0 as std::os::raw::c_int],
    [0 as std::os::raw::c_int, 1 as std::os::raw::c_int],
    [1 as std::os::raw::c_int, 0 as std::os::raw::c_int],
];
#[no_mangle]
pub unsafe extern "C" fn walk(mut y: std::os::raw::c_int, mut x: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut t: std::os::raw::c_int = 0;
    if y == 0 || y == h || x == 0 || x == w {
        cnt = cnt.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulonglong);
        return;
    }
    t = y * (w + 1 as std::os::raw::c_int) + x;
    let ref mut fresh0 = *grid.offset(t as isize);
    *fresh0 = (*fresh0).wrapping_add(1);
    *fresh0;
    let ref mut fresh1 = *grid.offset((len - t) as isize);
    *fresh1 = (*fresh1).wrapping_add(1);
    *fresh1;
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        if *grid.offset((t + next[i as usize]) as isize) == 0 {
            walk(
                y + dir[i as usize][0 as std::os::raw::c_int as usize],
                x + dir[i as usize][1 as std::os::raw::c_int as usize],
            );
        }
        i += 1;
        i;
    }
    let ref mut fresh2 = *grid.offset(t as isize);
    *fresh2 = (*fresh2).wrapping_sub(1);
    *fresh2;
    let ref mut fresh3 = *grid.offset((len - t) as isize);
    *fresh3 = (*fresh3).wrapping_sub(1);
    *fresh3;
}
#[no_mangle]
pub unsafe extern "C" fn solve(
    mut hh: std::os::raw::c_int,
    mut ww: std::os::raw::c_int,
    mut recur: std::os::raw::c_int,
) -> std::os::raw::c_ulonglong {
    let mut t: std::os::raw::c_int = 0;
    let mut cx: std::os::raw::c_int = 0;
    let mut cy: std::os::raw::c_int = 0;
    let mut x: std::os::raw::c_int = 0;
    h = hh;
    w = ww;
    if h & 1 as std::os::raw::c_int != 0 {
        t = w;
        w = h;
        h = t;
    }
    if h & 1 as std::os::raw::c_int != 0 {
        return 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    }
    if w == 1 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    }
    if w == 2 as std::os::raw::c_int {
        return h as std::os::raw::c_ulonglong;
    }
    if h == 2 as std::os::raw::c_int {
        return w as std::os::raw::c_ulonglong;
    }
    cy = h / 2 as std::os::raw::c_int;
    cx = w / 2 as std::os::raw::c_int;
    len = (h + 1 as std::os::raw::c_int) * (w + 1 as std::os::raw::c_int);
    grid = realloc(grid as *mut u8, len as std::os::raw::c_ulong) as *mut byte;
    let fresh4 = len;
    len = len - 1;
    memset(grid as *mut u8, 0 as std::os::raw::c_int, fresh4 as std::os::raw::c_ulong);
    next[0 as std::os::raw::c_int as usize] = -(1 as std::os::raw::c_int);
    next[1 as std::os::raw::c_int as usize] = -w - 1 as std::os::raw::c_int;
    next[2 as std::os::raw::c_int as usize] = 1 as std::os::raw::c_int;
    next[3 as std::os::raw::c_int as usize] = w + 1 as std::os::raw::c_int;
    if recur != 0 {
        cnt = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    }
    x = cx + 1 as std::os::raw::c_int;
    while x < w {
        t = cy * (w + 1 as std::os::raw::c_int) + x;
        *grid.offset(t as isize) = 1 as std::os::raw::c_int as byte;
        *grid.offset((len - t) as isize) = 1 as std::os::raw::c_int as byte;
        walk(cy - 1 as std::os::raw::c_int, x);
        x += 1;
        x;
    }
    cnt = cnt.wrapping_add(1);
    cnt;
    if h == w {
        cnt = cnt.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulonglong);
    } else if w & 1 as std::os::raw::c_int == 0 && recur != 0 {
        solve(w, h, 0 as std::os::raw::c_int);
    }
    return cnt;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut y: std::os::raw::c_int = 0;
    let mut x: std::os::raw::c_int = 0;
    y = 1 as std::os::raw::c_int;
    while y <= 10 as std::os::raw::c_int {
        x = 1 as std::os::raw::c_int;
        while x <= y {
            if x & 1 as std::os::raw::c_int == 0 || y & 1 as std::os::raw::c_int == 0 {
                printf(
                    b"%d x %d: %llu\n\0" as *const u8 as *const std::os::raw::c_char,
                    y,
                    x,
                    solve(y, x, 1 as std::os::raw::c_int),
                );
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

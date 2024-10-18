#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
}
impl std::default::Default for point {
    fn default() -> Self {
        point {
        x: std::os::raw::c_int::default(),
        y: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rot<'a1>(
    mut n: std::os::raw::c_int,
    mut p: Option<&'a1 mut crate::point>,
    mut rx: std::os::raw::c_int,
    mut ry: std::os::raw::c_int,
) {
    let mut t: std::os::raw::c_int = 0;
    if ry == 0 {
        if rx == 1 as std::os::raw::c_int {
            (*borrow_mut(&mut p).unwrap()).x = n - 1 as std::os::raw::c_int - (*borrow(& p).unwrap()).x;
            (*borrow_mut(&mut p).unwrap()).y = n - 1 as std::os::raw::c_int - (*borrow(& p).unwrap()).y;
        }
        t = (*borrow_mut(&mut p).unwrap()).x;
        (*borrow_mut(&mut p).unwrap()).x = (*borrow_mut(&mut p).unwrap()).y;
        (*borrow_mut(&mut p).unwrap()).y = t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn d2pt<'a1>(
    mut n: std::os::raw::c_int,
    mut d: std::os::raw::c_int,
    mut p: Option<&'a1 mut crate::point>,
) {
    let mut s: std::os::raw::c_int = 1 as std::os::raw::c_int;
    let mut t: std::os::raw::c_int = d;
    let mut rx: std::os::raw::c_int = 0;
    let mut ry: std::os::raw::c_int = 0;
    (*borrow_mut(&mut p).unwrap()).x = 0 as std::os::raw::c_int;
    (*borrow_mut(&mut p).unwrap()).y = 0 as std::os::raw::c_int;
    while s < n {
        rx = 1 as std::os::raw::c_int & t / 2 as std::os::raw::c_int;
        ry = 1 as std::os::raw::c_int & (t ^ rx);
        rot(s, borrow_mut(&mut p), rx, ry);
        (*borrow_mut(&mut p).unwrap()).x += s * rx;
        (*borrow_mut(&mut p).unwrap()).y += s * ry;
        t /= 4 as std::os::raw::c_int;
        s *= 2 as std::os::raw::c_int;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut d: std::os::raw::c_int = 0;
    let mut x: std::os::raw::c_int = 0;
    let mut y: std::os::raw::c_int = 0;
    let mut cx: std::os::raw::c_int = 0;
    let mut cy: std::os::raw::c_int = 0;
    let mut px: std::os::raw::c_int = 0;
    let mut py: std::os::raw::c_int = 0;
    let mut pts: [[std::os::raw::c_char; 96]; 96] = [[0; 96]; 96];
    let mut curr: point = point { x: 0, y: 0 };
    let mut prev: point = point { x: 0, y: 0 };
    x = 0 as std::os::raw::c_int;
    while x < 32 as std::os::raw::c_int * 3 as std::os::raw::c_int {
        y = 0 as std::os::raw::c_int;
        while y < 32 as std::os::raw::c_int * 3 as std::os::raw::c_int {
            pts[x as usize][y as usize] = ' ' as i32 as std::os::raw::c_char;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    prev.y = 0 as std::os::raw::c_int;
    prev.x = prev.y;
    pts[0 as std::os::raw::c_int
        as usize][0 as std::os::raw::c_int as usize] = '.' as i32 as std::os::raw::c_char;
    d = 1 as std::os::raw::c_int;
    while d < 32 as std::os::raw::c_int * 32 as std::os::raw::c_int {
        d2pt(32 as std::os::raw::c_int, d, Some(&mut curr));
        cx = curr.x * 3 as std::os::raw::c_int;
        cy = curr.y * 3 as std::os::raw::c_int;
        px = prev.x * 3 as std::os::raw::c_int;
        py = prev.y * 3 as std::os::raw::c_int;
        pts[cx as usize][cy as usize] = '.' as i32 as std::os::raw::c_char;
        if cx == px {
            if py < cy {
                y = py + 1 as std::os::raw::c_int;
                while y < cy {
                    pts[cx as usize][y as usize] = '|' as i32 as std::os::raw::c_char;
                    y += 1;
                    y;
                }
            } else {
                y = cy + 1 as std::os::raw::c_int;
                while y < py {
                    pts[cx as usize][y as usize] = '|' as i32 as std::os::raw::c_char;
                    y += 1;
                    y;
                }
            }
        } else if px < cx {
            x = px + 1 as std::os::raw::c_int;
            while x < cx {
                pts[x as usize][cy as usize] = '_' as i32 as std::os::raw::c_char;
                x += 1;
                x;
            }
        } else {
            x = cx + 1 as std::os::raw::c_int;
            while x < px {
                pts[x as usize][cy as usize] = '_' as i32 as std::os::raw::c_char;
                x += 1;
                x;
            }
        }
        prev = curr;
        d += 1;
        d;
    }
    x = 0 as std::os::raw::c_int;
    while x < 32 as std::os::raw::c_int * 3 as std::os::raw::c_int {
        y = 0 as std::os::raw::c_int;
        while y < 32 as std::os::raw::c_int * 3 as std::os::raw::c_int {
            printf(
                b"%c\0" as *const u8 as *const std::os::raw::c_char,
                pts[y as usize][x as usize] as std::os::raw::c_int,
            );
            y += 1;
            y;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        x += 1;
        x;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vec {
    pub x: std::os::raw::c_double,
    pub y: std::os::raw::c_double,
}
impl std::default::Default for vec {
    fn default() -> Self {
        vec {
        x: std::os::raw::c_double::default(),
        y: std::os::raw::c_double::default()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct polygon_t {
    pub n: std::os::raw::c_int,
    pub v: * mut crate::vec,
}
impl std::default::Default for polygon_t {
    fn default() -> Self {
        polygon_t {
        n: std::os::raw::c_int::default(),
        v: core::ptr::null_mut()
        }
    }
}

pub type polygon = * mut crate::polygon_t;
#[no_mangle]
pub extern "C" fn vsub(mut a: vec, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x - b.x;
    c.y = a.y - b.y;
    return c;
}
#[no_mangle]
pub extern "C" fn vadd(mut a: vec, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    return c;
}
#[no_mangle]
pub extern "C" fn vdot(mut a: vec, mut b: vec) -> std::os::raw::c_double {
    return a.x * b.x + a.y * b.y;
}
#[no_mangle]
pub extern "C" fn vcross(mut a: vec, mut b: vec) -> std::os::raw::c_double {
    return a.x * b.y - a.y * b.x;
}
#[no_mangle]
pub extern "C" fn vmadd(mut a: vec, mut s: std::os::raw::c_double, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x + s * b.x;
    c.y = a.y + s * b.y;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn intersect(
    mut x0: vec,
    mut x1: vec,
    mut y0: vec,
    mut y1: vec,
    mut tol: std::os::raw::c_double,
    mut sect: *mut vec,
) -> std::os::raw::c_int {
    let mut dx: vec = vsub(x1, x0);
    let mut dy: vec = vsub(y1, y0);
    let mut d: std::os::raw::c_double = vcross(dy, dx);
    let mut a: std::os::raw::c_double = 0.;
    if d == 0. {
        return 0 as std::os::raw::c_int;
    }
    a = (vcross(x0, dx) - vcross(y0, dx)) / d;
    if !sect.is_null() {
        *sect = vmadd(y0, a, dy);
    }
    if a < -tol || a > 1 as std::os::raw::c_int as std::os::raw::c_double + tol {
        return -(1 as std::os::raw::c_int);
    }
    if a < tol || a > 1 as std::os::raw::c_int as std::os::raw::c_double - tol {
        return 0 as std::os::raw::c_int;
    }
    a = (vcross(x0, dy) - vcross(y0, dy)) / d;
    if a < 0 as std::os::raw::c_int as std::os::raw::c_double || a > 1 as std::os::raw::c_int as std::os::raw::c_double {
        return -(1 as std::os::raw::c_int);
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dist(
    mut x: vec,
    mut y0: vec,
    mut y1: vec,
    mut tol: std::os::raw::c_double,
) -> std::os::raw::c_double {
    let mut dy: vec = vsub(y1, y0);
    let mut x1: vec = vec { x: 0., y: 0. };
    let mut s: vec = vec { x: 0., y: 0. };
    let mut r: std::os::raw::c_int = 0;
    x1.x = x.x + dy.y;
    x1.y = x.y - dy.x;
    r = intersect(x, x1, y0, y1, tol, &mut s);
    if r == -(1 as std::os::raw::c_int) {
        return std::f64::INFINITY;
    }
    s = vsub(s, x);
    return sqrt(vdot(s, s));
}
#[no_mangle]
pub unsafe extern "C" fn inside<'a1>(
    mut v: crate::vec,
    mut p: Option<&'a1 mut crate::polygon_t>,
    mut tol: std::os::raw::c_double,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut k: std::os::raw::c_int = 0;
    let mut crosses: std::os::raw::c_int = 0;
    let mut intersectResult: std::os::raw::c_int = 0;
    let mut pv: *mut vec = core::ptr::null_mut();
    let mut min_x: std::os::raw::c_double = 0.;
    let mut max_x: std::os::raw::c_double = 0.;
    let mut min_y: std::os::raw::c_double = 0.;
    let mut max_y: std::os::raw::c_double = 0.;
    i = 0 as std::os::raw::c_int;
    while i < (*borrow(& p).unwrap()).n {
        k = (i + 1 as std::os::raw::c_int) % (*borrow(& p).unwrap()).n;
        min_x = dist(v, *((*borrow_mut(&mut p).unwrap()).v).offset(i as isize), *((*borrow_mut(&mut p).unwrap()).v).offset(k as isize), tol);
        if min_x < tol {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
        i;
    }
    max_x = (*((*borrow_mut(&mut p).unwrap()).v).offset(0 as std::os::raw::c_int as isize)).x;
    min_x = max_x;
    max_y = (*((*borrow_mut(&mut p).unwrap()).v).offset(1 as std::os::raw::c_int as isize)).y;
    min_y = max_y;
    i = 0 as std::os::raw::c_int;
    pv = (*borrow_mut(&mut p).unwrap()).v;
    while i < (*borrow(& p).unwrap()).n {
        if (*pv).x > max_x {
            max_x = (*pv).x;
        }
        if (*pv).x < min_x {
            min_x = (*pv).x;
        }
        if (*pv).y > max_y {
            max_y = (*pv).y;
        }
        if (*pv).y < min_y {
            min_y = (*pv).y;
        }
        i += 1;
        i;
        pv = pv.offset(1);
        pv;
    }
    if v.x < min_x || v.x > max_x || v.y < min_y || v.y > max_y {
        return -(1 as std::os::raw::c_int);
    }
    max_x -= min_x;
    max_x *= 2 as std::os::raw::c_int as std::os::raw::c_double;
    max_y -= min_y;
    max_y *= 2 as std::os::raw::c_int as std::os::raw::c_double;
    max_x += max_y;
    let mut e: vec = vec { x: 0., y: 0. };
    loop {
        crosses = 0 as std::os::raw::c_int;
        e
            .x = v.x
            + (1 as std::os::raw::c_int as std::os::raw::c_double
                + rand() as std::os::raw::c_double
                    / (2147483647 as std::os::raw::c_int as std::os::raw::c_double + 1.0f64)) * max_x;
        e
            .y = v.y
            + (1 as std::os::raw::c_int as std::os::raw::c_double
                + rand() as std::os::raw::c_double
                    / (2147483647 as std::os::raw::c_int as std::os::raw::c_double + 1.0f64)) * max_x;
        i = 0 as std::os::raw::c_int;
        while i < (*borrow(& p).unwrap()).n {
            k = (i + 1 as std::os::raw::c_int) % (*borrow(& p).unwrap()).n;
            intersectResult = intersect(
                v,
                e,
                *((*borrow_mut(&mut p).unwrap()).v).offset(i as isize),
                *((*borrow_mut(&mut p).unwrap()).v).offset(k as isize),
                tol,
                core::ptr::null_mut(),
            );
            if intersectResult == 0 {
                break;
            }
            if intersectResult == 1 as std::os::raw::c_int {
                crosses += 1;
                crosses;
            }
            i += 1;
            i;
        }
        if i == (*borrow(& p).unwrap()).n {
            break;
        }
    }
    return if crosses & 1 as std::os::raw::c_int != 0 {
        1 as std::os::raw::c_int
    } else {
        -(1 as std::os::raw::c_int)
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut vsq: [vec; 8] = [
        {
            let mut init = vec {
                x: 0 as std::os::raw::c_int as std::os::raw::c_double,
                y: 0 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = vec {
                x: 10 as std::os::raw::c_int as std::os::raw::c_double,
                y: 0 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = vec {
                x: 10 as std::os::raw::c_int as std::os::raw::c_double,
                y: 10 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = vec {
                x: 0 as std::os::raw::c_int as std::os::raw::c_double,
                y: 10 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = vec { x: 2.5f64, y: 2.5f64 };
            init
        },
        {
            let mut init = vec { x: 7.5f64, y: 0.1f64 };
            init
        },
        {
            let mut init = vec { x: 7.5f64, y: 7.5f64 };
            init
        },
        {
            let mut init = vec { x: 2.5f64, y: 7.5f64 };
            init
        },
    ];
    let mut sq: polygon_t = {
        let mut init = polygon_t {
            n: 4 as std::os::raw::c_int,
            v: vsq.as_mut_ptr(),
        };
        init
    };
    let mut sq_hole: polygon_t = {
        let mut init = polygon_t {
            n: 8 as std::os::raw::c_int,
            v: vsq.as_mut_ptr(),
        };
        init
    };
    let mut c: vec = {
        let mut init = vec {
            x: 10 as std::os::raw::c_int as std::os::raw::c_double,
            y: 5 as std::os::raw::c_int as std::os::raw::c_double,
        };
        init
    };
    let mut d: vec = {
        let mut init = vec {
            x: 5 as std::os::raw::c_int as std::os::raw::c_double,
            y: 5 as std::os::raw::c_int as std::os::raw::c_double,
        };
        init
    };
    printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, inside(c, Some(&mut sq), 1e-10f64));
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        inside(c, Some(&mut sq_hole), 1e-10f64),
    );
    printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, inside(d, Some(&mut sq), 1e-10f64));
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        inside(d, Some(&mut sq_hole), 1e-10f64),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

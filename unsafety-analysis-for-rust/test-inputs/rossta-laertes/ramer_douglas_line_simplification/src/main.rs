#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point_tag {
    pub x: std::os::raw::c_double,
    pub y: std::os::raw::c_double,
}
impl std::default::Default for point_tag {
    fn default() -> Self {
        point_tag {
        x: std::os::raw::c_double::default(),
        y: std::os::raw::c_double::default()
        }
    }
}

pub type point_t = crate::point_tag;
#[no_mangle]
pub unsafe extern "C" fn perpendicular_distance(
    mut p: point_t,
    mut p1: point_t,
    mut p2: point_t,
) -> std::os::raw::c_double {
    let mut dx: std::os::raw::c_double = p2.x - p1.x;
    let mut dy: std::os::raw::c_double = p2.y - p1.y;
    let mut d: std::os::raw::c_double = sqrt(dx * dx + dy * dy);
    return fabs(p.x * dy - p.y * dx + p2.x * p1.y - p2.y * p1.x) / d;
}
#[no_mangle]
pub unsafe extern "C" fn douglas_peucker(
    mut points: *const point_t,
    mut n: size_t,
    mut epsilon: std::os::raw::c_double,
    mut dest: *mut point_t,
    mut destlen: size_t,
) -> size_t {
    if n >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {} else {
        __assert_fail(
            b"n >= 2\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            22 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 75],
                &[std::os::raw::c_char; 75],
            >(
                b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2158: {
        if n >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {} else {
            __assert_fail(
                b"n >= 2\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                22 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 75],
                    &[std::os::raw::c_char; 75],
                >(
                    b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if epsilon >= 0 as std::os::raw::c_int as std::os::raw::c_double {} else {
        __assert_fail(
            b"epsilon >= 0\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            23 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 75],
                &[std::os::raw::c_char; 75],
            >(
                b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2115: {
        if epsilon >= 0 as std::os::raw::c_int as std::os::raw::c_double {} else {
            __assert_fail(
                b"epsilon >= 0\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                23 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 75],
                    &[std::os::raw::c_char; 75],
                >(
                    b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut max_dist: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut index: size_t = 0 as std::os::raw::c_int as size_t;
    let mut i: size_t = 1 as std::os::raw::c_int as size_t;
    while i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) < n {
        let mut dist: std::os::raw::c_double = perpendicular_distance(
            *points.offset(i as isize),
            *points.offset(0 as std::os::raw::c_int as isize),
            *points.offset(n.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize),
        );
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    if max_dist > epsilon {
        let mut n1: size_t = douglas_peucker(
            points,
            index.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            epsilon,
            dest,
            destlen,
        );
        if destlen >= n1.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
            destlen = (destlen as std::os::raw::c_ulong)
                .wrapping_sub(n1.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong))
                as size_t as size_t;
            dest = dest
                .offset(n1.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize);
        } else {
            destlen = 0 as std::os::raw::c_int as size_t;
        }
        let mut n2: size_t = douglas_peucker(
            points.offset(index as isize),
            n.wrapping_sub(index),
            epsilon,
            dest,
            destlen,
        );
        return n1.wrapping_add(n2).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    }
    if destlen >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        *dest
            .offset(
                0 as std::os::raw::c_int as isize,
            ) = *points.offset(0 as std::os::raw::c_int as isize);
        *dest
            .offset(
                1 as std::os::raw::c_int as isize,
            ) = *points
            .offset(n.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize);
    }
    return 2 as std::os::raw::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn print_points(mut points: *const point_t, mut n: size_t) {
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < n {
        if i > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            printf(b" \0" as *const u8 as *const std::os::raw::c_char);
        }
        printf(
            b"(%g, %g)\0" as *const u8 as *const std::os::raw::c_char,
            (*points.offset(i as isize)).x,
            (*points.offset(i as isize)).y,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut points: [point_t; 10] = [
        {
            let mut init = point_tag {
                x: 0 as std::os::raw::c_int as std::os::raw::c_double,
                y: 0 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 1 as std::os::raw::c_int as std::os::raw::c_double,
                y: 0.1f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 2 as std::os::raw::c_int as std::os::raw::c_double,
                y: -0.1f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 3 as std::os::raw::c_int as std::os::raw::c_double,
                y: 5 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 4 as std::os::raw::c_int as std::os::raw::c_double,
                y: 6 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 5 as std::os::raw::c_int as std::os::raw::c_double,
                y: 7 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 6 as std::os::raw::c_int as std::os::raw::c_double,
                y: 8.1f64,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 7 as std::os::raw::c_int as std::os::raw::c_double,
                y: 9 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 8 as std::os::raw::c_int as std::os::raw::c_double,
                y: 9 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
        {
            let mut init = point_tag {
                x: 9 as std::os::raw::c_int as std::os::raw::c_double,
                y: 9 as std::os::raw::c_int as std::os::raw::c_double,
            };
            init
        },
    ];
    let len: size_t = (std::mem::size_of::<[point_t; 10]>() as std::os::raw::c_ulong)
        .wrapping_div(std::mem::size_of::<point_t>() as std::os::raw::c_ulong);
    let vla = len as usize;
    let mut out: Vec::<point_t> = ::std::vec::from_elem(point_t { x: 0., y: 0. }, vla);
    let mut n: size_t = douglas_peucker(
        points.as_mut_ptr(),
        len,
        1.0f64,
        out.as_mut_ptr(),
        len,
    );
    print_points(out.as_mut_ptr(), n);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

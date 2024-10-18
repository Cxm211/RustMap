#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: std::os::raw::c_double,
    pub y: std::os::raw::c_double,
}
impl std::default::Default for point {
    fn default() -> Self {
        point {
        x: std::os::raw::c_double::default(),
        y: std::os::raw::c_double::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn distance(mut p1: point, mut p2: point) -> std::os::raw::c_double {
    return sqrt((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y));
}
#[no_mangle]
pub unsafe extern "C" fn findCircles(
    mut p1: point,
    mut p2: point,
    mut radius: std::os::raw::c_double,
) {
    let mut separation: std::os::raw::c_double = distance(p1, p2);
    let mut mirrorDistance: std::os::raw::c_double = 0.;
    if separation == 0.0f64 {
        if radius == 0.0f64 {
            printf(
                b"\nNo circles can be drawn through (%.4f,%.4f)\0" as *const u8
                    as *const std::os::raw::c_char,
                p1.x,
                p1.y,
            );
        } else {
            printf(
                b"\nInfinitely many circles can be drawn through (%.4f,%.4f)\0"
                    as *const u8 as *const std::os::raw::c_char,
                p1.x,
                p1.y,
            );
        };
    } else if separation == 2 as std::os::raw::c_int as std::os::raw::c_double * radius {
        printf(
            b"\nGiven points are opposite ends of a diameter of the circle with center (%.4f,%.4f) and radius %.4f\0"
                as *const u8 as *const std::os::raw::c_char,
            (p1.x + p2.x) / 2 as std::os::raw::c_int as std::os::raw::c_double,
            (p1.y + p2.y) / 2 as std::os::raw::c_int as std::os::raw::c_double,
            radius,
        );
    } else if separation > 2 as std::os::raw::c_int as std::os::raw::c_double * radius {
        printf(
            b"\nGiven points are farther away from each other than a diameter of a circle with radius %.4f\0"
                as *const u8 as *const std::os::raw::c_char,
            radius,
        );
    } else {
        mirrorDistance = sqrt(
            pow(radius, 2 as std::os::raw::c_int as std::os::raw::c_double)
                - pow(
                    separation / 2 as std::os::raw::c_int as std::os::raw::c_double,
                    2 as std::os::raw::c_int as std::os::raw::c_double,
                ),
        );
        printf(b"\nTwo circles are possible.\0" as *const u8 as *const std::os::raw::c_char);
        printf(
            b"\nCircle C1 with center (%.4f,%.4f), radius %.4f and Circle C2 with center (%.4f,%.4f), radius %.4f\0"
                as *const u8 as *const std::os::raw::c_char,
            (p1.x + p2.x) / 2 as std::os::raw::c_int as std::os::raw::c_double
                + mirrorDistance * (p1.y - p2.y) / separation,
            (p1.y + p2.y) / 2 as std::os::raw::c_int as std::os::raw::c_double
                + mirrorDistance * (p2.x - p1.x) / separation,
            radius,
            (p1.x + p2.x) / 2 as std::os::raw::c_int as std::os::raw::c_double
                - mirrorDistance * (p1.y - p2.y) / separation,
            (p1.y + p2.y) / 2 as std::os::raw::c_int as std::os::raw::c_double
                - mirrorDistance * (p2.x - p1.x) / separation,
            radius,
        );
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut cases: [point; 10] = [
        {
            let mut init = point {
                x: 0.1234f64,
                y: 0.9876f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.8765f64,
                y: 0.2345f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.0000f64,
                y: 2.0000f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.0000f64,
                y: 0.0000f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.1234f64,
                y: 0.9876f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.1234f64,
                y: 0.9876f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.1234f64,
                y: 0.9876f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.8765f64,
                y: 0.2345f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.1234f64,
                y: 0.9876f64,
            };
            init
        },
        {
            let mut init = point {
                x: 0.1234f64,
                y: 0.9876f64,
            };
            init
        },
    ];
    let mut radii: [std::os::raw::c_double; 5] = [2.0f64, 1.0f64, 2.0f64, 0.5f64, 0.0f64];
    i = 0 as std::os::raw::c_int;
    while i < 5 as std::os::raw::c_int {
        printf(
            b"\nCase %d)\0" as *const u8 as *const std::os::raw::c_char,
            i + 1 as std::os::raw::c_int,
        );
        findCircles(
            cases[(2 as std::os::raw::c_int * i) as usize],
            cases[(2 as std::os::raw::c_int * i + 1 as std::os::raw::c_int) as usize],
            radii[i as usize],
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

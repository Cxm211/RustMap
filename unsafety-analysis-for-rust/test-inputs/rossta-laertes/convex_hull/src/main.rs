#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn qsort(
        __base: *mut u8,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = std::os::raw::c_ulong;
pub type __compar_fn_t = Option<unsafe extern "C"  fn(_: * const u8,_: * const u8,) -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tPoint {
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
}
impl std::default::Default for tPoint {
    fn default() -> Self {
        tPoint {
        x: std::os::raw::c_int::default(),
        y: std::os::raw::c_int::default()
        }
    }
}

pub type Point = crate::tPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tNode {
    pub data: crate::tPoint,
    pub next: * mut crate::tNode,
}
impl std::default::Default for tNode {
    fn default() -> Self {
        tNode {
        data: crate::tPoint::default(),
        next: core::ptr::null_mut()
        }
    }
}

pub type Node = crate::tNode;
#[no_mangle]
pub unsafe extern "C" fn ccw<'a1, 'a2, 'a3>(
    mut a: Option<&'a1 crate::tPoint>,
    mut b: Option<&'a2 crate::tPoint>,
    mut c: Option<&'a3 crate::tPoint>,
) -> bool {
    return ((*(b).clone().unwrap()).x - (*(a).clone().unwrap()).x) * ((*(c).clone().unwrap()).y - (*(a).clone().unwrap()).y) > ((*(b).clone().unwrap()).y - (*(a).clone().unwrap()).y) * ((*(c).clone().unwrap()).x - (*(a).clone().unwrap()).x);
}
#[no_mangle]
pub unsafe extern "C" fn comp(
    mut lhs: *const u8,
    mut rhs: *const u8,
) -> std::os::raw::c_int {
    let mut lp: Point = *(lhs as *mut Point);
    let mut rp: Point = *(rhs as *mut Point);
    if lp.x < rp.x {
        return -(1 as std::os::raw::c_int);
    }
    if rp.x < lp.x {
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeNode(mut ptr: *mut Node) {
    if ptr.is_null() {
        return;
    }
    freeNode((*ptr).next);
    (*ptr).next = core::ptr::null_mut();
    free(ptr as *mut u8);
}
#[no_mangle]
pub unsafe extern "C" fn pushBack(mut ptr: *mut Node, mut data: Point) -> *mut Node {
    let mut tmp: *mut Node = ptr;
    if ptr.is_null() {
        ptr = malloc(std::mem::size_of::<Node>() as std::os::raw::c_ulong) as *mut Node;
        (*ptr).data = data;
        (*ptr).next = core::ptr::null_mut();
        return ptr;
    }
    while !((*tmp).next).is_null() {
        tmp = (*tmp).next;
    }
    (*tmp).next = malloc(std::mem::size_of::<Node>() as std::os::raw::c_ulong) as *mut Node;
    (*(*tmp).next).data = data;
    (*(*tmp).next).next = core::ptr::null_mut();
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn popBack(mut ptr: *mut Node) -> *mut Node {
    let mut tmp: *mut Node = ptr;
    if ptr.is_null() {
        return core::ptr::null_mut();
    }
    if ((*ptr).next).is_null() {
        free(ptr as *mut u8);
        return core::ptr::null_mut();
    }
    while !((*(*tmp).next).next).is_null() {
        tmp = (*tmp).next;
    }
    free((*tmp).next as *mut u8);
    (*tmp).next = core::ptr::null_mut();
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn print(mut ptr: *mut Node) {
    printf(b"[\0" as *const u8 as *const std::os::raw::c_char);
    if !ptr.is_null() {
        printf(
            b"(%d, %d)\0" as *const u8 as *const std::os::raw::c_char,
            (*ptr).data.x,
            (*ptr).data.y,
        );
        ptr = (*ptr).next;
    }
    while !ptr.is_null() {
        printf(
            b", (%d, %d)\0" as *const u8 as *const std::os::raw::c_char,
            (*ptr).data.x,
            (*ptr).data.y,
        );
        ptr = (*ptr).next;
    }
    printf(b"]\0" as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn convexHull(
    mut len: std::os::raw::c_int,
    mut p: *mut Point,
) -> *mut Node {
    let mut h: *mut Node = core::ptr::null_mut();
    let mut hptr: *mut Node = core::ptr::null_mut();
    let mut hLen: size_t = 0 as std::os::raw::c_int as size_t;
    let mut i: std::os::raw::c_int = 0;
    qsort(
        p as *mut u8,
        len as size_t,
        std::mem::size_of::<Point>() as std::os::raw::c_ulong,
        Some(
            comp
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    i = 0 as std::os::raw::c_int;
    while i < len {
        while hLen >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            hptr = h;
            while !((*(*hptr).next).next).is_null() {
                hptr = (*hptr).next;
            }
            if ccw(
                borrow(& Some(&mut (*hptr).data)),
                borrow(& Some(&mut (*(*hptr).next).data)),
                borrow(& Some(&mut *p.offset(i as isize))),
            ) {
                break;
            }
            h = popBack(h);
            hLen = hLen.wrapping_sub(1);
            hLen;
        }
        h = pushBack(h, *p.offset(i as isize));
        hLen = hLen.wrapping_add(1);
        hLen;
        i += 1;
        i;
    }
    i = len - 1 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        while hLen >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            hptr = h;
            while !((*(*hptr).next).next).is_null() {
                hptr = (*hptr).next;
            }
            if ccw(
                borrow(& Some(&mut (*hptr).data)),
                borrow(& Some(&mut (*(*hptr).next).data)),
                borrow(& Some(&mut *p.offset(i as isize))),
            ) {
                break;
            }
            h = popBack(h);
            hLen = hLen.wrapping_sub(1);
            hLen;
        }
        h = pushBack(h, *p.offset(i as isize));
        hLen = hLen.wrapping_add(1);
        hLen;
        i -= 1;
        i;
    }
    popBack(h);
    return h;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut points: [Point; 20] = [
        {
            let mut init = tPoint {
                x: 16 as std::os::raw::c_int,
                y: 3 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 12 as std::os::raw::c_int,
                y: 17 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 0 as std::os::raw::c_int,
                y: 6 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(4 as std::os::raw::c_int),
                y: -(6 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 16 as std::os::raw::c_int,
                y: 6 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 16 as std::os::raw::c_int,
                y: -(7 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 16 as std::os::raw::c_int,
                y: -(3 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 17 as std::os::raw::c_int,
                y: -(4 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 5 as std::os::raw::c_int,
                y: 19 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 19 as std::os::raw::c_int,
                y: -(8 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 3 as std::os::raw::c_int,
                y: 16 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 12 as std::os::raw::c_int,
                y: 13 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: 3 as std::os::raw::c_int,
                y: -(4 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 17 as std::os::raw::c_int,
                y: 5 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(3 as std::os::raw::c_int),
                y: 15 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(3 as std::os::raw::c_int),
                y: -(9 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 0 as std::os::raw::c_int,
                y: 11 as std::os::raw::c_int,
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(9 as std::os::raw::c_int),
                y: -(3 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: -(4 as std::os::raw::c_int),
                y: -(2 as std::os::raw::c_int),
            };
            init
        },
        {
            let mut init = tPoint {
                x: 12 as std::os::raw::c_int,
                y: 10 as std::os::raw::c_int,
            };
            init
        },
    ];
    let mut hull: *mut Node = convexHull(
        (std::mem::size_of::<[Point; 20]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<Point>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
        points.as_mut_ptr(),
    );
    printf(b"Convex Hull: \0" as *const u8 as *const std::os::raw::c_char);
    print(hull);
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    freeNode(hull);
    hull = core::ptr::null_mut();
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

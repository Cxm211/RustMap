#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matrix_t {
    pub h: std::os::raw::c_int,
    pub w: std::os::raw::c_int,
    pub x: * mut std::os::raw::c_double,
}
impl std::default::Default for matrix_t {
    fn default() -> Self {
        matrix_t {
        h: std::os::raw::c_int::default(),
        w: std::os::raw::c_int::default(),
        x: 0 as * mut std::os::raw::c_double
        }
    }
}

pub type matrix = * mut crate::matrix_t;
#[no_mangle]
pub unsafe extern "C" fn dot(
    mut a: *mut std::os::raw::c_double,
    mut b: *mut std::os::raw::c_double,
    mut len: std::os::raw::c_int,
    mut step: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut r: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = a;
        a = a.offset(1);
        r += *fresh1 * *b;
        b = b.offset(step as isize);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mat_new(mut h: std::os::raw::c_int, mut w: std::os::raw::c_int) -> matrix {
    let mut r: matrix = malloc(
        (std::mem::size_of::<matrix_t>() as std::os::raw::c_ulong)
            .wrapping_add(
                (std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong)
                    .wrapping_mul(w as std::os::raw::c_ulong)
                    .wrapping_mul(h as std::os::raw::c_ulong),
            ),
    ) as matrix;
    (*r).h = h;
    (*r).w = w;
    (*r).x = r.offset(1 as std::os::raw::c_int as isize) as *mut std::os::raw::c_double;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mat_mul<'a1, 'a2>(mut a: Option<&'a1 mut crate::matrix_t>, mut b: Option<&'a2 mut crate::matrix_t>) -> * mut crate::matrix_t {
    let mut r: matrix = core::ptr::null_mut();
    let mut p: *mut std::os::raw::c_double = core::ptr::null_mut();
    let mut pa: *mut std::os::raw::c_double = core::ptr::null_mut();
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    if (*borrow(& a).unwrap()).w != (*borrow(& b).unwrap()).h {
        return core::ptr::null_mut();
    }
    r = mat_new((*borrow_mut(&mut a).unwrap()).h, (*borrow_mut(&mut b).unwrap()).w);
    p = (*r).x;
    pa = (*borrow_mut(&mut a).unwrap()).x;
    i = 0 as std::os::raw::c_int;
    while i < (*borrow(& a).unwrap()).h {
        j = 0 as std::os::raw::c_int;
        while j < (*borrow(& b).unwrap()).w {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = dot(pa, ((*borrow_mut(&mut b).unwrap()).x).offset(j as isize), (*borrow_mut(&mut a).unwrap()).w, (*borrow_mut(&mut b).unwrap()).w);
            j += 1;
            j;
        }
        i += 1;
        i;
        pa = pa.offset((*borrow_mut(&mut a).unwrap()).w as isize);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mat_show(mut a: matrix) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut p: *mut std::os::raw::c_double = (*a).x;
    i = 0 as std::os::raw::c_int;
    while i < (*a).h {
        j = 0 as std::os::raw::c_int;
        while j < (*a).w {
            let fresh3 = p;
            p = p.offset(1);
            printf(b"\t%7.3f\0" as *const u8 as *const std::os::raw::c_char, *fresh3);
            j += 1;
            j;
        }
        i += 1;
        i;
        putchar('\n' as i32);
    }
    putchar('\n' as i32);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut da: [std::os::raw::c_double; 16] = [
        1 as std::os::raw::c_int as std::os::raw::c_double,
        1 as std::os::raw::c_int as std::os::raw::c_double,
        1 as std::os::raw::c_int as std::os::raw::c_double,
        1 as std::os::raw::c_int as std::os::raw::c_double,
        2 as std::os::raw::c_int as std::os::raw::c_double,
        4 as std::os::raw::c_int as std::os::raw::c_double,
        8 as std::os::raw::c_int as std::os::raw::c_double,
        16 as std::os::raw::c_int as std::os::raw::c_double,
        3 as std::os::raw::c_int as std::os::raw::c_double,
        9 as std::os::raw::c_int as std::os::raw::c_double,
        27 as std::os::raw::c_int as std::os::raw::c_double,
        81 as std::os::raw::c_int as std::os::raw::c_double,
        4 as std::os::raw::c_int as std::os::raw::c_double,
        16 as std::os::raw::c_int as std::os::raw::c_double,
        64 as std::os::raw::c_int as std::os::raw::c_double,
        256 as std::os::raw::c_int as std::os::raw::c_double,
    ];
    let mut db: [std::os::raw::c_double; 12] = [
        4.0f64,
        -3.0f64,
        4.0f64 / 3 as std::os::raw::c_int as std::os::raw::c_double,
        -13.0f64 / 3 as std::os::raw::c_int as std::os::raw::c_double,
        19.0f64 / 4 as std::os::raw::c_int as std::os::raw::c_double,
        -7.0f64 / 3 as std::os::raw::c_int as std::os::raw::c_double,
        3.0f64 / 2 as std::os::raw::c_int as std::os::raw::c_double,
        -2.0f64,
        7.0f64 / 6 as std::os::raw::c_int as std::os::raw::c_double,
        -1.0f64 / 6 as std::os::raw::c_int as std::os::raw::c_double,
        1.0f64 / 4 as std::os::raw::c_int as std::os::raw::c_double,
        -1.0f64 / 6 as std::os::raw::c_int as std::os::raw::c_double,
    ];
    let mut a: matrix_t = {
        let mut init = matrix_t {
            h: 4 as std::os::raw::c_int,
            w: 4 as std::os::raw::c_int,
            x: da.as_mut_ptr(),
        };
        init
    };
    let mut b: matrix_t = {
        let mut init = matrix_t {
            h: 4 as std::os::raw::c_int,
            w: 3 as std::os::raw::c_int,
            x: db.as_mut_ptr(),
        };
        init
    };
    let mut c: matrix = mat_mul(Some(&mut a), Some(&mut b));
    mat_show(c);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

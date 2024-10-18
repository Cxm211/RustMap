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
pub struct vcount {
    pub v: std::os::raw::c_double,
    pub c: std::os::raw::c_int,
}
impl std::default::Default for vcount {
    fn default() -> Self {
        vcount {
        v: std::os::raw::c_double::default(),
        c: std::os::raw::c_int::default()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmp_dbl(
    mut a: *const u8,
    mut b: *const u8,
) -> std::os::raw::c_int {
    let mut x: std::os::raw::c_double = *(a as *const std::os::raw::c_double)
        - *(b as *const std::os::raw::c_double);
    return if x < 0 as std::os::raw::c_int as std::os::raw::c_double {
        -(1 as std::os::raw::c_int)
    } else {
        (x > 0 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn vc_cmp(
    mut a: *const u8,
    mut b: *const u8,
) -> std::os::raw::c_int {
    return (*(b as *const vcount)).c - (*(a as *const vcount)).c;
}
#[no_mangle]
pub unsafe extern "C" fn get_mode<'a1>(
    mut x: * mut std::os::raw::c_double,
    mut len_0: std::os::raw::c_int,
    mut list: Option<&'a1 mut * mut crate::vcount>,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut vc: *mut vcount = 0 as *mut vcount;
    qsort(
        x as *mut u8,
        len_0 as size_t,
        std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong,
        Some(
            cmp_dbl
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    i = 0 as std::os::raw::c_int;
    j = 1 as std::os::raw::c_int;
    while i < len_0 - 1 as std::os::raw::c_int {
        i += 1;
        i;
        j
            += (*x.offset(i as isize) != *x.offset((i + 1 as std::os::raw::c_int) as isize))
                as std::os::raw::c_int;
    }
    vc = malloc(
        (std::mem::size_of::<vcount>() as std::os::raw::c_ulong)
            .wrapping_mul(j as std::os::raw::c_ulong),
    ) as *mut vcount;
    *borrow_mut(&mut list).unwrap() = vc;
    (*vc.offset(0 as std::os::raw::c_int as isize)).v = *x.offset(0 as std::os::raw::c_int as isize);
    (*vc.offset(0 as std::os::raw::c_int as isize)).c = 1 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    i = j;
    while i < len_0 - 1 as std::os::raw::c_int {
        if *x.offset(i as isize) != *x.offset((i + 1 as std::os::raw::c_int) as isize) {
            j += 1;
            (*vc.offset(j as isize)).v = *x.offset((i + 1 as std::os::raw::c_int) as isize);
        }
        i += 1;
        i;
        let ref mut fresh0 = (*vc.offset(j as isize)).c;
        *fresh0 += 1;
        *fresh0;
    }
    qsort(
        vc as *mut u8,
        (j + 1 as std::os::raw::c_int) as size_t,
        std::mem::size_of::<vcount>() as std::os::raw::c_ulong,
        Some(
            vc_cmp
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    i = 0 as std::os::raw::c_int;
    while i <= j
        && (*vc.offset(i as isize)).c == (*vc.offset(0 as std::os::raw::c_int as isize)).c
    {
        i += 1;
        i;
    }
    return i;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut values: [std::os::raw::c_double; 13] = [
        1 as std::os::raw::c_int as std::os::raw::c_double,
        3 as std::os::raw::c_int as std::os::raw::c_double,
        6 as std::os::raw::c_int as std::os::raw::c_double,
        6 as std::os::raw::c_int as std::os::raw::c_double,
        6 as std::os::raw::c_int as std::os::raw::c_double,
        6 as std::os::raw::c_int as std::os::raw::c_double,
        7 as std::os::raw::c_int as std::os::raw::c_double,
        7 as std::os::raw::c_int as std::os::raw::c_double,
        12 as std::os::raw::c_int as std::os::raw::c_double,
        12 as std::os::raw::c_int as std::os::raw::c_double,
        12 as std::os::raw::c_int as std::os::raw::c_double,
        12 as std::os::raw::c_int as std::os::raw::c_double,
        17 as std::os::raw::c_int as std::os::raw::c_double,
    ];
    let mut vc: *mut vcount = 0 as *mut vcount;
    let mut i: std::os::raw::c_int = 0;
    let mut n_modes: std::os::raw::c_int = get_mode(
        values.as_mut_ptr(),
        (std::mem::size_of::<[std::os::raw::c_double; 13]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
        Some(&mut vc),
    );
    printf(b"got %d modes:\n\0" as *const u8 as *const std::os::raw::c_char, n_modes);
    i = 0 as std::os::raw::c_int;
    while i < n_modes {
        printf(
            b"\tvalue = %g, count = %d\n\0" as *const u8 as *const std::os::raw::c_char,
            (*vc.offset(i as isize)).v,
            (*vc.offset(i as isize)).c,
        );
        i += 1;
        i;
    }
    free(vc as *mut u8);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

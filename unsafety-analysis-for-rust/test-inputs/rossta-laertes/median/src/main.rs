#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
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
pub struct floatList {
    pub list: * mut std::os::raw::c_float,
    pub size: std::os::raw::c_int,
}
impl std::default::Default for floatList {
    fn default() -> Self {
        floatList {
        list: 0 as * mut std::os::raw::c_float,
        size: std::os::raw::c_int::default()
        }
    }
}

pub type FloatList = * mut crate::floatList;
#[no_mangle]
pub unsafe extern "C" fn floatcmp(
    mut a: *const u8,
    mut b: *const u8,
) -> std::os::raw::c_int {
    if *(a as *const std::os::raw::c_float) < *(b as *const std::os::raw::c_float) {
        return -(1 as std::os::raw::c_int)
    } else {
        return (*(a as *const std::os::raw::c_float) > *(b as *const std::os::raw::c_float))
            as std::os::raw::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn median<'a1>(mut fl: Option<&'a1 mut crate::floatList>) -> std::os::raw::c_float {
    qsort(
        (*borrow_mut(&mut fl).unwrap()).list as *mut u8,
        (*borrow_mut(&mut fl).unwrap()).size as size_t,
        std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong,
        Some(
            floatcmp
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    return (0.5f64
        * (*((*borrow_mut(&mut fl).unwrap()).list).offset(((*borrow(& fl).unwrap()).size / 2 as std::os::raw::c_int) as isize)
            + *((*borrow_mut(&mut fl).unwrap()).list)
                .offset((((*borrow(& fl).unwrap()).size - 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int) as isize))
            as std::os::raw::c_double) as std::os::raw::c_float;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut floats1: [std::os::raw::c_float; 6] = [
        5.1f64 as std::os::raw::c_float,
        2.6f64 as std::os::raw::c_float,
        6.2f64 as std::os::raw::c_float,
        8.8f64 as std::os::raw::c_float,
        4.6f64 as std::os::raw::c_float,
        4.1f64 as std::os::raw::c_float,
    ];
    let mut flist1: floatList = {
        let mut init = floatList {
            list: floats1.as_mut_ptr(),
            size: (std::mem::size_of::<[std::os::raw::c_float; 6]>() as std::os::raw::c_ulong)
                .wrapping_div(std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
                as std::os::raw::c_int,
        };
        init
    };
    let mut floats2: [std::os::raw::c_float; 5] = [
        5.1f64 as std::os::raw::c_float,
        2.6f64 as std::os::raw::c_float,
        8.8f64 as std::os::raw::c_float,
        4.6f64 as std::os::raw::c_float,
        4.1f64 as std::os::raw::c_float,
    ];
    let mut flist2: floatList = {
        let mut init = floatList {
            list: floats2.as_mut_ptr(),
            size: (std::mem::size_of::<[std::os::raw::c_float; 5]>() as std::os::raw::c_ulong)
                .wrapping_div(std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
                as std::os::raw::c_int,
        };
        init
    };
    printf(
        b"flist1 median is %7.2f\n\0" as *const u8 as *const std::os::raw::c_char,
        median(Some(&mut flist1)) as std::os::raw::c_double,
    );
    printf(
        b"flist2 median is %7.2f\n\0" as *const u8 as *const std::os::raw::c_char,
        median(Some(&mut flist2)) as std::os::raw::c_double,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

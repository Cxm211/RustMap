#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
}
pub type __builtin_va_list = [crate::__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: * mut u8,
    pub reg_save_area: * mut u8,
}
impl std::default::Default for __va_list_tag {
    fn default() -> Self {
        __va_list_tag {
        gp_offset: std::os::raw::c_uint::default(),
        fp_offset: std::os::raw::c_uint::default(),
        overflow_arg_area: 0 as * mut u8,
        reg_save_area: 0 as * mut u8
        }
    }
}

pub type va_list = [crate::__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sma_obj {
    pub sma: std::os::raw::c_double,
    pub sum: std::os::raw::c_double,
    pub period: std::os::raw::c_int,
    pub values: * mut std::os::raw::c_double,
    pub lv: std::os::raw::c_int,
}
impl std::default::Default for sma_obj {
    fn default() -> Self {
        sma_obj {
        sma: std::os::raw::c_double::default(),
        sum: std::os::raw::c_double::default(),
        period: std::os::raw::c_int::default(),
        values: 0 as * mut std::os::raw::c_double,
        lv: std::os::raw::c_int::default()
        }
    }
}

pub type sma_obj_t = crate::sma_obj;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sma_result {
    pub handle: *mut sma_obj_t,
    pub sma: std::os::raw::c_double,
    pub values: *mut std::os::raw::c_double,
}
pub type sma_result_t = crate::sma_result;
pub type Action = std::os::raw::c_uint;
pub const SMA_MEAN: Action = 4;
pub const SMA_ADD: Action = 3;
pub const SMA_VALUES: Action = 2;
pub const SMA_FREE: Action = 1;
pub const SMA_NEW: Action = 0;
#[no_mangle]
pub unsafe extern "C" fn sma(mut action: Action, mut args: ...) -> sma_result_t {
    let mut vl: std::ffi::VaListImpl;
    let mut r: sma_result_t = sma_result {
        handle: 0 as *mut sma_obj_t,
    };
    let mut o: *mut sma_obj_t = 0 as *mut sma_obj_t;
    let mut v_0: std::os::raw::c_double = 0.;
    vl = args.clone();
    match action as std::os::raw::c_uint {
        0 => {
            r
                .handle = malloc(std::mem::size_of::<sma_obj_t>() as std::os::raw::c_ulong)
                as *mut sma_obj_t;
            (*r.handle).sma = 0.0f64;
            (*r.handle).period = vl.arg::<std::os::raw::c_int>();
            (*r.handle)
                .values = malloc(
                ((*r.handle).period as std::os::raw::c_ulong)
                    .wrapping_mul(
                        std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong,
                    ),
            ) as *mut std::os::raw::c_double;
            (*r.handle).lv = 0 as std::os::raw::c_int;
            (*r.handle).sum = 0.0f64;
        }
        1 => {
            r.handle = vl.arg::<*mut sma_obj_t>();
            free((*r.handle).values as *mut u8);
            free(r.handle as *mut u8);
            r.handle = 0 as *mut sma_obj_t;
        }
        2 => {
            o = vl.arg::<*mut sma_obj_t>();
            r.values = (*o).values;
        }
        4 => {
            o = vl.arg::<*mut sma_obj_t>();
            r.sma = (*o).sma;
        }
        3 => {
            o = vl.arg::<*mut sma_obj_t>();
            v_0 = vl.arg::<std::os::raw::c_double>();
            if (*o).lv < (*o).period {
                let fresh0 = (*o).lv;
                (*o).lv = (*o).lv + 1;
                *((*o).values).offset(fresh0 as isize) = v_0;
                (*o).sum += v_0;
                (*o).sma = (*o).sum / (*o).lv as std::os::raw::c_double;
            } else {
                (*o).sum -= *((*o).values).offset(((*o).lv % (*o).period) as isize);
                (*o).sum += v_0;
                (*o).sma = (*o).sum / (*o).period as std::os::raw::c_double;
                *((*o).values).offset(((*o).lv % (*o).period) as isize) = v_0;
                (*o).lv += 1;
                (*o).lv;
            }
            r.sma = (*o).sma;
        }
        _ => {}
    }
    return r;
}
#[no_mangle]
pub static mut v: [std::os::raw::c_double; 10] = [
    1 as std::os::raw::c_int as std::os::raw::c_double,
    2 as std::os::raw::c_int as std::os::raw::c_double,
    3 as std::os::raw::c_int as std::os::raw::c_double,
    4 as std::os::raw::c_int as std::os::raw::c_double,
    5 as std::os::raw::c_int as std::os::raw::c_double,
    5 as std::os::raw::c_int as std::os::raw::c_double,
    4 as std::os::raw::c_int as std::os::raw::c_double,
    3 as std::os::raw::c_int as std::os::raw::c_double,
    2 as std::os::raw::c_int as std::os::raw::c_double,
    1 as std::os::raw::c_int as std::os::raw::c_double,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut h3: *mut sma_obj_t = (sma(SMA_NEW, 3 as std::os::raw::c_int)).handle;
    let mut h5: *mut sma_obj_t = (sma(SMA_NEW, 5 as std::os::raw::c_int)).handle;
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (std::mem::size_of::<[std::os::raw::c_double; 10]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong)
    {
        printf(
            b"next number %lf, SMA_3 = %lf, SMA_5 = %lf\n\0" as *const u8
                as *const std::os::raw::c_char,
            v[i as usize],
            (sma(SMA_ADD, h3, v[i as usize])).sma,
            (sma(SMA_ADD, h5, v[i as usize])).sma,
        );
        i += 1;
        i;
    }
    sma(SMA_FREE, h3);
    sma(SMA_FREE, h5);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

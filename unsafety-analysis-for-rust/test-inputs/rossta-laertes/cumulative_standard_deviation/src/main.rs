#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub type size_t = std::os::raw::c_ulong;
pub type Action = std::os::raw::c_uint;
pub const COUNT: Action = 3;
pub const VAR: Action = 2;
pub const MEAN: Action = 1;
pub const STDDEV: Action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_obj_struct {
    pub sum: std::os::raw::c_double,
    pub sum2: std::os::raw::c_double,
    pub num: std::os::raw::c_ulong,
    pub action: std::os::raw::c_uint,
}
impl std::default::Default for stat_obj_struct {
    fn default() -> Self {
        stat_obj_struct {
        sum: std::os::raw::c_double::default(),
        sum2: std::os::raw::c_double::default(),
        num: std::os::raw::c_ulong::default(),
        action: std::os::raw::c_uint::default()
        }
    }
}

pub type sStatObject = crate::stat_obj_struct;
pub type StatObject = * mut crate::stat_obj_struct;
#[no_mangle]
pub unsafe extern "C" fn NewStatObject(mut action: Action) -> StatObject {
    let mut so: StatObject = core::ptr::null_mut();
    so = malloc(std::mem::size_of::<sStatObject>() as std::os::raw::c_ulong) as StatObject;
    (*so).sum = 0.0f64;
    (*so).sum2 = 0.0f64;
    (*so).num = 0 as std::os::raw::c_int as size_t;
    (*so).action = action;
    return so;
}
#[no_mangle]
pub unsafe extern "C" fn stat_obj_value(
    mut so: StatObject,
    mut action: Action,
) -> std::os::raw::c_double {
    let mut num: std::os::raw::c_double = 0.;
    let mut mean: std::os::raw::c_double = 0.;
    let mut var: std::os::raw::c_double = 0.;
    let mut stddev: std::os::raw::c_double = 0.;
    if (*so).num as std::os::raw::c_double == 0.0f64 {
        return 0.0f64;
    }
    num = (*so).num as std::os::raw::c_double;
    if action as std::os::raw::c_uint == COUNT as std::os::raw::c_int as std::os::raw::c_uint {
        return num;
    }
    mean = (*so).sum / num;
    if action as std::os::raw::c_uint == MEAN as std::os::raw::c_int as std::os::raw::c_uint {
        return mean;
    }
    var = (*so).sum2 / num - mean * mean;
    if action as std::os::raw::c_uint == VAR as std::os::raw::c_int as std::os::raw::c_uint {
        return var;
    }
    stddev = sqrt(var);
    if action as std::os::raw::c_uint == STDDEV as std::os::raw::c_int as std::os::raw::c_uint {
        return stddev;
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn stat_object_add(
    mut so: StatObject,
    mut v_0: std::os::raw::c_double,
) -> std::os::raw::c_double {
    (*so).num = ((*so).num).wrapping_add(1);
    (*so).num;
    (*so).sum += v_0;
    (*so).sum2 += v_0 * v_0;
    return stat_obj_value(so, (*so).action);
}
#[no_mangle]
pub static mut v: [std::os::raw::c_double; 8] = [
    2 as std::os::raw::c_int as std::os::raw::c_double,
    4 as std::os::raw::c_int as std::os::raw::c_double,
    4 as std::os::raw::c_int as std::os::raw::c_double,
    4 as std::os::raw::c_int as std::os::raw::c_double,
    5 as std::os::raw::c_int as std::os::raw::c_double,
    5 as std::os::raw::c_int as std::os::raw::c_double,
    7 as std::os::raw::c_int as std::os::raw::c_double,
    9 as std::os::raw::c_int as std::os::raw::c_double,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut so: StatObject = NewStatObject(STDDEV);
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (std::mem::size_of::<[std::os::raw::c_double; 8]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong)
    {
        printf(
            b"val: %lf  std dev: %lf\n\0" as *const u8 as *const std::os::raw::c_char,
            v[i as usize],
            stat_object_add(so, v[i as usize]),
        );
        i += 1;
        i;
    }
    free(so as *mut u8);
    so = core::ptr::null_mut();
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

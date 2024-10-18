#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn asin(_: f64) -> f64;
    fn sin(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct double_to_double {
    pub fn_0: Option<unsafe extern "C" fn(*mut double_to_double, f64) -> f64>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compose_functor {
    pub fn_0: Option<unsafe extern "C" fn(*mut compose_functor, f64) -> f64>,
    pub f: *mut double_to_double,
    pub g: *mut double_to_double,
}
#[no_mangle]
pub extern "C" fn compose_call(mut this: *mut compose_functor, mut x: f64) -> f64 {
    unsafe {
        return ((*(*this).f).fn_0).expect("non-null function pointer")(
            (*this).f,
            ((*(*this).g).fn_0).expect("non-null function pointer")((*this).g, x),
        );
    }
}

#[no_mangle]
pub extern "C" fn compose(
    mut f: *mut double_to_double,
    mut g: *mut double_to_double,
) -> *mut double_to_double {
    unsafe {
        let mut result: *mut compose_functor =
            malloc(::core::mem::size_of::<compose_functor>() as u64) as *mut compose_functor;
        (*result).fn_0 =
            Some(compose_call as unsafe extern "C" fn(*mut compose_functor, f64) -> f64);
        (*result).f = f;
        (*result).g = g;
        return result as *mut double_to_double;
    }
}

#[no_mangle]
pub extern "C" fn sin_call(mut this: *mut double_to_double, mut x: f64) -> f64 {
    unsafe {
        return sin(x);
    }
}

#[no_mangle]
pub extern "C" fn asin_call(mut this: *mut double_to_double, mut x: f64) -> f64 {
    unsafe {
        return asin(x);
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut my_sin: *mut double_to_double =
            malloc(::core::mem::size_of::<double_to_double>() as u64) as *mut double_to_double;
        (*my_sin).fn_0 = Some(sin_call as unsafe extern "C" fn(*mut double_to_double, f64) -> f64);
        let mut my_asin: *mut double_to_double =
            malloc(::core::mem::size_of::<double_to_double>() as u64) as *mut double_to_double;
        (*my_asin).fn_0 =
            Some(asin_call as unsafe extern "C" fn(*mut double_to_double, f64) -> f64);
        let mut sin_asin: *mut double_to_double = compose(my_sin, my_asin);
        print!(
            "{}\n",
            ((*sin_asin).fn_0).expect("non-null function pointer")(sin_asin, 0.5f64)
        );
        free(sin_asin as *mut libc::c_void);
        free(my_sin as *mut libc::c_void);
        free(my_asin as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

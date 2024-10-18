#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub type seq_func = Option<unsafe extern "C"  fn(_: * mut u8,) -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gen_t {
    pub f: Option<unsafe extern "C"  fn(_: * mut u8,) -> std::os::raw::c_int>,
    pub output: std::os::raw::c_int,
}
impl std::default::Default for gen_t {
    fn default() -> Self {
        gen_t {
        f: None,
        output: std::os::raw::c_int::default()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct power_gen_t {
    pub f: Option<unsafe extern "C"  fn(_: * mut u8,) -> std::os::raw::c_int>,
    pub output: std::os::raw::c_int,
    pub pos: std::os::raw::c_int,
    pub n: std::os::raw::c_int,
}
impl std::default::Default for power_gen_t {
    fn default() -> Self {
        power_gen_t {
        f: None,
        output: std::os::raw::c_int::default(),
        pos: std::os::raw::c_int::default(),
        n: std::os::raw::c_int::default()
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_gen_t {
    pub f: Option<unsafe extern "C"  fn(_: * mut u8,) -> std::os::raw::c_int>,
    pub output: std::os::raw::c_int,
    pub in_0: * mut u8,
    pub without: * mut u8,
}
impl std::default::Default for filter_gen_t {
    fn default() -> Self {
        filter_gen_t {
        f: None,
        output: std::os::raw::c_int::default(),
        in_0: 0 as * mut u8,
        without: 0 as * mut u8
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn seq_next(mut state: *mut u8) -> std::os::raw::c_int {
    let ref mut fresh0 = (*(state as *mut gen_t)).output;
    *fresh0 = (*(state as *mut seq_func)).expect("non-null function pointer")(state);
    return *fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn power_next(mut s: *mut u8) -> std::os::raw::c_int {
    let ref mut fresh1 = (*(s as *mut power_gen_t)).pos;
    *fresh1 += 1;
    return pow(*fresh1 as std::os::raw::c_double, (*(s as *mut power_gen_t)).n as std::os::raw::c_double)
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn power_seq(mut n: std::os::raw::c_int) -> *mut u8 {
    let mut s: *mut power_gen_t = malloc(
        std::mem::size_of::<power_gen_t>() as std::os::raw::c_ulong,
    ) as *mut power_gen_t;
    (*s).output = -(1 as std::os::raw::c_int);
    (*s).f = Some(power_next as unsafe extern "C" fn(*mut u8) -> std::os::raw::c_int);
    (*s).n = n;
    (*s).pos = -(1 as std::os::raw::c_int);
    return s as *mut u8;
}
#[no_mangle]
pub unsafe extern "C" fn filter_next(mut s: *mut u8) -> std::os::raw::c_int {
    let mut in_0: *mut gen_t = (*(s as *mut filter_gen_t)).in_0 as *mut gen_t;
    let mut wo: *mut gen_t = (*(s as *mut filter_gen_t)).without as *mut gen_t;
    loop {
        seq_next(in_0 as *mut u8);
        while (*wo).output < (*in_0).output {
            seq_next(wo as *mut u8);
        }
        if !((*wo).output == (*in_0).output) {
            break;
        }
    }
    return (*in_0).output;
}
#[no_mangle]
pub unsafe extern "C" fn filter_seq(
    mut in_0: *mut gen_t,
    mut without: *mut gen_t,
) -> *mut u8 {
    let mut filt: *mut filter_gen_t = malloc(
        std::mem::size_of::<filter_gen_t>() as std::os::raw::c_ulong,
    ) as *mut filter_gen_t;
    (*filt).in_0 = in_0 as *mut u8;
    (*filt).without = without as *mut u8;
    (*filt)
        .f = Some(filter_next as unsafe extern "C" fn(*mut u8) -> std::os::raw::c_int);
    (*filt).output = -(1 as std::os::raw::c_int);
    return filt as *mut u8;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut s: *mut u8 = filter_seq(
        power_seq(2 as std::os::raw::c_int) as *mut gen_t,
        power_seq(3 as std::os::raw::c_int) as *mut gen_t,
    );
    i = 0 as std::os::raw::c_int;
    while i < 20 as std::os::raw::c_int {
        seq_next(s);
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, seq_next(s));
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

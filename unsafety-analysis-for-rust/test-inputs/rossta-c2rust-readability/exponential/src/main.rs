#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type seq_func = Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gen_t {
    pub f: seq_func,
    pub output: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct power_gen_t {
    pub f: seq_func,
    pub output: libc::c_int,
    pub pos: libc::c_int,
    pub n: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_gen_t {
    pub f: seq_func,
    pub output: libc::c_int,
    pub in_0: *mut libc::c_void,
    pub without: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn seq_next(mut state: *mut libc::c_void) -> libc::c_int {
    let ref mut fresh0 = (*(state as *mut gen_t)).output;
    *fresh0 = (*(state as *mut seq_func)).expect("non-null function pointer")(state);
    return *fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn power_next(mut s: *mut libc::c_void) -> libc::c_int {
    let ref mut fresh1 = (*(s as *mut power_gen_t)).pos;
    *fresh1 += 1;
    return pow(*fresh1 as libc::c_double, (*(s as *mut power_gen_t)).n as libc::c_double)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn power_seq(mut n: libc::c_int) -> *mut libc::c_void {
    let mut s: *mut power_gen_t = malloc(
        ::core::mem::size_of::<power_gen_t>() as libc::c_ulong,
    ) as *mut power_gen_t;
    (*s).output = -(1 as libc::c_int);
    (*s).f = Some(power_next as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int);
    (*s).n = n;
    (*s).pos = -(1 as libc::c_int);
    return s as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn filter_next(mut s: *mut libc::c_void) -> libc::c_int {
    let mut in_0: *mut gen_t = (*(s as *mut filter_gen_t)).in_0 as *mut gen_t;
    let mut wo: *mut gen_t = (*(s as *mut filter_gen_t)).without as *mut gen_t;
    loop {
        seq_next(in_0 as *mut libc::c_void);
        while (*wo).output < (*in_0).output {
            seq_next(wo as *mut libc::c_void);
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
) -> *mut libc::c_void {
    let mut filt: *mut filter_gen_t = malloc(
        ::core::mem::size_of::<filter_gen_t>() as libc::c_ulong,
    ) as *mut filter_gen_t;
    (*filt).in_0 = in_0 as *mut libc::c_void;
    (*filt).without = without as *mut libc::c_void;
    (*filt)
        .f = Some(filter_next as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int);
    (*filt).output = -(1 as libc::c_int);
    return filt as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_void = filter_seq(
        power_seq(2 as libc::c_int) as *mut gen_t,
        power_seq(3 as libc::c_int) as *mut gen_t,
    );
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        seq_next(s);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        printf(b"%d\n\0" as *const u8 as *const libc::c_char, seq_next(s));
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

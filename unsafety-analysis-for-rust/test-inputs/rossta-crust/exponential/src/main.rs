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
    fn pow(_: f64, _: f64) -> f64;
}
pub type seq_func = Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gen_t {
    pub f: seq_func,
    pub output: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct power_gen_t {
    pub f: seq_func,
    pub output: i32,
    pub pos: i32,
    pub n: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_gen_t {
    pub f: seq_func,
    pub output: i32,
    pub in_0: *mut libc::c_void,
    pub without: *mut libc::c_void,
}
#[no_mangle]
pub extern "C" fn seq_next(mut state: *mut libc::c_void) -> i32 {
    unsafe {
        let ref mut fresh0 = (*(state as *mut gen_t)).output;
        *fresh0 = (*(state as *mut seq_func)).expect("non-null function pointer")(state);
        return *fresh0;
    }
}

#[no_mangle]
pub extern "C" fn power_next(mut s: *mut libc::c_void) -> i32 {
    unsafe {
        let ref mut fresh1 = (*(s as *mut power_gen_t)).pos;
        *fresh1 += 1;
        return pow(*fresh1 as f64, (*(s as *mut power_gen_t)).n as f64) as i32;
    }
}

#[no_mangle]
pub extern "C" fn power_seq(mut n: i32) -> *mut libc::c_void {
    unsafe {
        let mut s: *mut power_gen_t =
            malloc(::core::mem::size_of::<power_gen_t>() as u64) as *mut power_gen_t;
        (*s).output = -1;
        (*s).f = Some(power_next as unsafe extern "C" fn(*mut libc::c_void) -> i32);
        (*s).n = n;
        (*s).pos = -1;
        return s as *mut libc::c_void;
    }
}

#[no_mangle]
pub extern "C" fn filter_next(mut s: *mut libc::c_void) -> i32 {
    unsafe {
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
}

#[no_mangle]
pub extern "C" fn filter_seq(mut in_0: *mut gen_t, mut without: *mut gen_t) -> *mut libc::c_void {
    unsafe {
        let mut filt: *mut filter_gen_t =
            malloc(::core::mem::size_of::<filter_gen_t>() as u64) as *mut filter_gen_t;
        (*filt).in_0 = in_0 as *mut libc::c_void;
        (*filt).without = without as *mut libc::c_void;
        (*filt).f = Some(filter_next as unsafe extern "C" fn(*mut libc::c_void) -> i32);
        (*filt).output = -1;
        return filt as *mut libc::c_void;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut s: *mut libc::c_void =
            filter_seq(power_seq(2) as *mut gen_t, power_seq(3) as *mut gen_t);
        i = 0;
        while i < 20 {
            seq_next(s);
            i += 1;
            i;
        }
        i = 0;
        while i < 10 {
            print!("{}\n", seq_next(s));
            i += 1;
            i;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

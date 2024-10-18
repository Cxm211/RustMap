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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn array_concat(
    mut a_0: *const libc::c_void,
    mut an: u64,
    mut b_0: *const libc::c_void,
    mut bn: u64,
    mut s: u64,
) -> *mut libc::c_void {
    unsafe {
        let mut p: *mut i8 = malloc(s.wrapping_mul(an.wrapping_add(bn))) as *mut i8;
        memcpy(p as *mut libc::c_void, a_0, an.wrapping_mul(s));
        memcpy(
            p.offset(an.wrapping_mul(s) as isize) as *mut libc::c_void,
            b_0,
            bn.wrapping_mul(s),
        );
        return p as *mut libc::c_void;
    }
}

#[no_mangle]
pub static mut a: [i32; 5] = [1, 2, 3, 4, 5];
#[no_mangle]
pub static mut b: [i32; 5] = [6, 7, 8, 9, 0];
fn main_0() -> i32 {
    unsafe {
        let mut i: u32 = 0;
        let mut c: *mut i32 = array_concat(
            a.as_ptr() as *const libc::c_void,
            5,
            b.as_ptr() as *const libc::c_void,
            5,
            ::core::mem::size_of::<i32>() as u64,
        ) as *mut i32;
        i = 0;
        while i < 10 {
            print!("{}\n", *c.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        free(c as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

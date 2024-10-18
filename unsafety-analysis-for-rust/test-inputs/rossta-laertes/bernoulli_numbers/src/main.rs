#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn bernoulli(mut rop: std::os::raw::c_int, mut n: std::os::raw::c_uint) {
    let mut m: std::os::raw::c_uint = 0;
    let mut j: std::os::raw::c_uint = 0;
    let mut i: size_t = 0;
    m = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while m <= n {
        j = m;
        while j > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            j = j.wrapping_sub(1);
            j;
        }
        m = m.wrapping_add(1);
        m;
    }
    let mut i_0: size_t = 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

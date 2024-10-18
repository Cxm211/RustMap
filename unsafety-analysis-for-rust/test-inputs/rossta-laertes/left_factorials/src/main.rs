#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn free(_: *mut u8);
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub extern "C" fn mpz_left_fac_ui(mut rop: std::os::raw::c_int, mut op: std::os::raw::c_ulong) {
    let mut i: size_t = 0;
    i = 1 as std::os::raw::c_int as size_t;
    while i <= op {
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpz_digitcount(mut op: std::os::raw::c_int) -> size_t {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut ret: size_t = strlen(t);
    free(t as *mut u8);
    return ret;
}
 fn main_0() -> std::os::raw::c_int {
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i <= 110 as std::os::raw::c_int as std::os::raw::c_ulong {
        i <= 10 as std::os::raw::c_int as std::os::raw::c_ulong
            || i.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_ulong)
                == 0 as std::os::raw::c_int as std::os::raw::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    i = 1000 as std::os::raw::c_int as size_t;
    while i <= 10000 as std::os::raw::c_int as std::os::raw::c_ulong {
        i = (i as std::os::raw::c_ulong).wrapping_add(1000 as std::os::raw::c_int as std::os::raw::c_ulong)
            as size_t as size_t;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn scanf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn log2(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn makehist(
    mut S: *mut std::os::raw::c_char,
    mut hist: *mut std::os::raw::c_int,
    mut len: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut wherechar: [std::os::raw::c_int; 256] = [0; 256];
    let mut i: std::os::raw::c_int = 0;
    let mut histlen: std::os::raw::c_int = 0;
    histlen = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        wherechar[i as usize] = -(1 as std::os::raw::c_int);
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < len {
        if wherechar[*S.offset(i as isize) as std::os::raw::c_int as usize]
            == -(1 as std::os::raw::c_int)
        {
            wherechar[*S.offset(i as isize) as std::os::raw::c_int as usize] = histlen;
            histlen += 1;
            histlen;
        }
        let ref mut fresh0 = *hist
            .offset(wherechar[*S.offset(i as isize) as std::os::raw::c_int as usize] as isize);
        *fresh0 += 1;
        *fresh0;
        i += 1;
        i;
    }
    return histlen;
}
#[no_mangle]
pub unsafe extern "C" fn entropy(
    mut hist: *mut std::os::raw::c_int,
    mut histlen: std::os::raw::c_int,
    mut len: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut i: std::os::raw::c_int = 0;
    let mut H: std::os::raw::c_double = 0.;
    H = 0 as std::os::raw::c_int as std::os::raw::c_double;
    i = 0 as std::os::raw::c_int;
    while i < histlen {
        H
            -= *hist.offset(i as isize) as std::os::raw::c_double / len as std::os::raw::c_double
                * log2(
                    *hist.offset(i as isize) as std::os::raw::c_double / len as std::os::raw::c_double,
                );
        i += 1;
        i;
    }
    return H;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut S: [std::os::raw::c_char; 100] = [0; 100];
    let mut len: std::os::raw::c_int = 0;
    let mut hist: *mut std::os::raw::c_int = core::ptr::null_mut();
    let mut histlen: std::os::raw::c_int = 0;
    let mut H: std::os::raw::c_double = 0.;
    scanf(b"%[^\n]\0" as *const u8 as *const std::os::raw::c_char, S.as_mut_ptr());
    len = strlen(S.as_mut_ptr()) as std::os::raw::c_int;
    hist = calloc(
        len as std::os::raw::c_ulong,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    histlen = makehist(S.as_mut_ptr(), hist, len);
    H = entropy(hist, histlen, len);
    printf(b"%lf\n\0" as *const u8 as *const std::os::raw::c_char, H);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

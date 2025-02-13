#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn snprintf(
        _: *mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: *const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    fn strtoul(
        _: *const std::os::raw::c_char,
        _: *mut *mut std::os::raw::c_char,
        _: std::os::raw::c_int,
    ) -> std::os::raw::c_ulong;
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn swap(
    mut str: *mut std::os::raw::c_char,
    mut i: std::os::raw::c_int,
    mut j: std::os::raw::c_int,
) {
    let mut c: std::os::raw::c_char = *str.offset(i as isize);
    *str.offset(i as isize) = *str.offset(j as isize);
    *str.offset(j as isize) = c;
}
#[no_mangle]
pub unsafe extern "C" fn reverse(
    mut str: *mut std::os::raw::c_char,
    mut i: std::os::raw::c_int,
    mut j: std::os::raw::c_int,
) {
    while i < j {
        swap(str, i, j);
        i += 1;
        i;
        j -= 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn next_permutation(mut str: *mut std::os::raw::c_char) -> bool {
    let mut len: std::os::raw::c_int = strlen(str) as std::os::raw::c_int;
    if len < 2 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int != 0;
    }
    let mut i: std::os::raw::c_int = len - 1 as std::os::raw::c_int;
    while i > 0 as std::os::raw::c_int {
        let mut j: std::os::raw::c_int = i;
        let mut k: std::os::raw::c_int = 0;
        i -= 1;
        if (*str.offset(i as isize) as std::os::raw::c_int)
            < *str.offset(j as isize) as std::os::raw::c_int
        {
            k = len;
            loop {
                k -= 1;
                if !(*str.offset(i as isize) as std::os::raw::c_int
                    >= *str.offset(k as isize) as std::os::raw::c_int)
                {
                    break;
                }
            }
            swap(str, i, k);
            reverse(str, j, len - 1 as std::os::raw::c_int);
            return 1 as std::os::raw::c_int != 0;
        }
    }
    return 0 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn next_highest_int(mut n: uint32_t) -> uint32_t {
    let mut str: [std::os::raw::c_char; 16] = [0; 16];
    snprintf(
        str.as_mut_ptr(),
        ::core::mem::size_of::<[std::os::raw::c_char; 16]>() as std::os::raw::c_ulong,
        b"%u\0" as *const u8 as *const std::os::raw::c_char,
        n,
    );
    if !next_permutation(str.as_mut_ptr()) {
        return 0 as std::os::raw::c_int as uint32_t;
    }
    return strtoul(str.as_mut_ptr(), 0 as *mut *mut std::os::raw::c_char, 10 as std::os::raw::c_int)
        as uint32_t;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut numbers: [uint32_t; 8] = [
        0 as std::os::raw::c_int as uint32_t,
        9 as std::os::raw::c_int as uint32_t,
        12 as std::os::raw::c_int as uint32_t,
        21 as std::os::raw::c_int as uint32_t,
        12453 as std::os::raw::c_int as uint32_t,
        738440 as std::os::raw::c_int as uint32_t,
        45072010 as std::os::raw::c_int as uint32_t,
        95322020 as std::os::raw::c_int as uint32_t,
    ];
    let count: std::os::raw::c_int = (::core::mem::size_of::<[uint32_t; 8]>() as std::os::raw::c_ulong)
        .wrapping_div(::core::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < count {
        printf(
            b"%d -> %d\n\0" as *const u8 as *const std::os::raw::c_char,
            numbers[i as usize],
            next_highest_int(numbers[i as usize]),
        );
        i += 1;
        i;
    }
    let big: [std::os::raw::c_char; 23] = *::core::mem::transmute::<
        &[u8; 23],
        &[std::os::raw::c_char; 23],
    >(b"9589776899767587796600\0");
    let mut next: [std::os::raw::c_char; 23] = [0; 23];
    memcpy(
        next.as_mut_ptr() as *mut std::os::raw::c_void,
        big.as_ptr() as *const std::os::raw::c_void,
        ::core::mem::size_of::<[std::os::raw::c_char; 23]>() as std::os::raw::c_ulong,
    );
    next_permutation(next.as_mut_ptr());
    printf(
        b"%s -> %s\n\0" as *const u8 as *const std::os::raw::c_char,
        big.as_ptr(),
        next.as_mut_ptr(),
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

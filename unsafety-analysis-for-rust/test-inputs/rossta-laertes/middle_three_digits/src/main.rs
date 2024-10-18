#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn mid3(mut n: std::os::raw::c_int) -> *mut std::os::raw::c_char {
    static mut buf: [std::os::raw::c_char; 32] = [0; 32];
    let mut l: std::os::raw::c_int = 0;
    sprintf(
        buf.as_mut_ptr(),
        b"%d\0" as *const u8 as *const std::os::raw::c_char,
        if n > 0 as std::os::raw::c_int { n } else { -n },
    );
    l = strlen(buf.as_mut_ptr()) as std::os::raw::c_int;
    if l < 3 as std::os::raw::c_int || l & 1 as std::os::raw::c_int == 0 {
        return 0 as *mut std::os::raw::c_char;
    }
    l = l / 2 as std::os::raw::c_int - 1 as std::os::raw::c_int;
    buf[(l + 3 as std::os::raw::c_int) as usize] = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return buf.as_mut_ptr().offset(l as isize);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: [std::os::raw::c_int; 18] = [
        123 as std::os::raw::c_int,
        12345 as std::os::raw::c_int,
        1234567 as std::os::raw::c_int,
        987654321 as std::os::raw::c_int,
        10001 as std::os::raw::c_int,
        -(10001 as std::os::raw::c_int),
        -(123 as std::os::raw::c_int),
        -(100 as std::os::raw::c_int),
        100 as std::os::raw::c_int,
        -(12345 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        -(10 as std::os::raw::c_int),
        2002 as std::os::raw::c_int,
        -(2002 as std::os::raw::c_int),
        0 as std::os::raw::c_int,
        1234567890 as std::os::raw::c_int,
    ];
    let mut i: std::os::raw::c_int = 0;
    let mut m: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong)
        < (std::mem::size_of::<[std::os::raw::c_int; 18]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
    {
        m = mid3(x[i as usize]);
        if m.is_null() {
            m = b"error\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
        }
        printf(b"%d: %s\n\0" as *const u8 as *const std::os::raw::c_char, x[i as usize], m);
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

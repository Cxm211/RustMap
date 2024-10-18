#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn setlocale(
        __category: std::os::raw::c_int,
        __locale: *const std::os::raw::c_char,
    ) -> *mut std::os::raw::c_char;
}
pub type wchar_t = std::os::raw::c_int;
#[no_mangle]
pub static mut s_suits: [wchar_t; 5] = unsafe {
    *std::mem::transmute::<
        &[u8; 20],
        &mut [wchar_t; 5],
    >(b"c&\0\0f&\0\0e&\0\0`&\0\0\0\0\0\0")
};
#[no_mangle]
pub static mut s_nums: [wchar_t; 14] = unsafe {
    *std::mem::transmute::<
        &[u8; 56],
        &mut [wchar_t; 14],
    >(
        b"A\0\0\x002\0\0\x003\0\0\x004\0\0\x005\0\0\x006\0\0\x007\0\0\08\0\0\09\0\0\0T\0\0\0J\0\0\0Q\0\0\0K\0\0\0\0\0\0\0",
    )
};
static mut seed: std::os::raw::c_int = 1 as std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn rnd() -> std::os::raw::c_int {
    seed = ((seed * 214013 as std::os::raw::c_int + 2531011 as std::os::raw::c_int) as std::os::raw::c_uint
        & ((1 as std::os::raw::c_uint) << 31 as std::os::raw::c_int)
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)) as std::os::raw::c_int;
    return seed >> 16 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn srnd(mut x: std::os::raw::c_int) {
    seed = x;
}
#[no_mangle]
pub unsafe extern "C" fn show(mut c: *const std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 52 as std::os::raw::c_int {
        printf(
            b"  \x1B[%dm%lc\x1B[m%lc\0" as *const u8 as *const std::os::raw::c_char,
            32 as std::os::raw::c_int
                - (1 as std::os::raw::c_int + *c) % 4 as std::os::raw::c_int / 2 as std::os::raw::c_int,
            s_suits[(*c % 4 as std::os::raw::c_int) as usize],
            s_nums[(*c / 4 as std::os::raw::c_int) as usize],
        );
        i += 1;
        if i % 8 as std::os::raw::c_int == 0 || i == 52 as std::os::raw::c_int {
            putchar('\n' as i32);
        }
        c = c.offset(1);
        c;
    }
}
#[no_mangle]
pub unsafe extern "C" fn deal(mut s: std::os::raw::c_int, mut t: *mut std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    srnd(s);
    i = 0 as std::os::raw::c_int;
    while i < 52 as std::os::raw::c_int {
        *t.offset(i as isize) = 51 as std::os::raw::c_int - i;
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 51 as std::os::raw::c_int {
        j = 51 as std::os::raw::c_int - rnd() % (52 as std::os::raw::c_int - i);
        s = *t.offset(i as isize);
        *t.offset(i as isize) = *t.offset(j as isize);
        *t.offset(j as isize) = s;
        i += 1;
        i;
    }
}
unsafe fn main_0(mut c: std::os::raw::c_int, mut v: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut s: std::os::raw::c_int = 0;
    let mut card: [std::os::raw::c_int; 52] = [0; 52];
    if c < 2 as std::os::raw::c_int
        || {
            s = atoi(*v.offset(1 as std::os::raw::c_int as isize));
            s <= 0 as std::os::raw::c_int
        }
    {
        s = 11982 as std::os::raw::c_int;
    }
    setlocale(6 as std::os::raw::c_int, b"\0" as *const u8 as *const std::os::raw::c_char);
    deal(s, card.as_mut_ptr());
    printf(b"Hand %d\n\0" as *const u8 as *const std::os::raw::c_char, s);
    show(card.as_mut_ptr());
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

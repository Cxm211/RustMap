#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn f(
    mut n: std::os::raw::c_int,
    mut x: std::os::raw::c_int,
    mut y: std::os::raw::c_int,
) -> std::os::raw::c_int {
    return (x + y * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) % n;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut n: std::os::raw::c_int = 0;
    if argc != 2 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int;
    }
    n = atoi(*argv.offset(1 as std::os::raw::c_int as isize));
    if n < 3 as std::os::raw::c_int || n % 2 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 2 as std::os::raw::c_int;
    }
    i = 0 as std::os::raw::c_int;
    while i < n {
        j = 0 as std::os::raw::c_int;
        while j < n {
            printf(
                b"% 4d\0" as *const u8 as *const std::os::raw::c_char,
                f(n, n - j - 1 as std::os::raw::c_int, i) * n + f(n, j, i) + 1 as std::os::raw::c_int,
            );
            j += 1;
            j;
        }
        putchar('\n' as i32);
        i += 1;
        i;
    }
    printf(
        b"\n Magic Constant: %d.\n\0" as *const u8 as *const std::os::raw::c_char,
        (n * n + 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int * n,
    );
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

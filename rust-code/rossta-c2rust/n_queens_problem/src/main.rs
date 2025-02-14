#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn abs(_: std::os::raw::c_int) -> std::os::raw::c_int;
}
#[no_mangle]
pub static mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn solve(
    mut n: std::os::raw::c_int,
    mut col: std::os::raw::c_int,
    mut hist: *mut std::os::raw::c_int,
) {
    if col == n {
        count += 1;
        printf(b"\nNo. %d\n-----\n\0" as *const u8 as *const std::os::raw::c_char, count);
        let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
        while i < n {
            let mut j: std::os::raw::c_int = 0 as std::os::raw::c_int;
            while j < n {
                putchar(
                    if j == *hist.offset(i as isize) {
                        'Q' as i32
                    } else if i + j & 1 as std::os::raw::c_int != 0 {
                        ' ' as i32
                    } else {
                        '.' as i32
                    },
                );
                j += 1;
                j;
            }
            i += 1;
            i;
            putchar('\n' as i32);
        }
        return;
    }
    let mut i_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut j_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i_0 < n {
        j_0 = 0 as std::os::raw::c_int;
        while j_0 < col
            && !(*hist.offset(j_0 as isize) == i_0
                || abs(*hist.offset(j_0 as isize) - i_0) == col - j_0)
        {
            j_0 += 1;
            j_0;
        }
        if !(j_0 < col) {
            *hist.offset(col as isize) = i_0;
            solve(n, col + 1 as std::os::raw::c_int, hist);
        }
        i_0 += 1;
        i_0;
    }
}
unsafe fn main_0(mut n: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char) -> std::os::raw::c_int {
    if n <= 1 as std::os::raw::c_int
        || {
            n = atoi(*argv.offset(1 as std::os::raw::c_int as isize));
            n <= 0 as std::os::raw::c_int
        }
    {
        n = 8 as std::os::raw::c_int;
    }
    let vla = n as usize;
    let mut hist: Vec::<std::os::raw::c_int> = ::std::vec::from_elem(0, vla);
    solve(n, 0 as std::os::raw::c_int, hist.as_mut_ptr());
    return 0;
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
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as std::os::raw::c_int,
                args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
            ) as i32,
        )
    }
}

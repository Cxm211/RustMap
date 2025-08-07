#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
}
pub type uint = std::os::raw::c_uint;
#[no_mangle]
pub unsafe extern "C" fn is_prime(mut n: uint) -> std::os::raw::c_int {
    if n.wrapping_rem(2 as std::os::raw::c_int as std::os::raw::c_uint) == 0
        || n.wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint) == 0
    {
        return 0 as std::os::raw::c_int;
    }
    let mut p: uint = 1 as std::os::raw::c_int as uint;
    while p.wrapping_mul(p) < n {
        p = (p as std::os::raw::c_uint).wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_uint) as uint
            as uint;
        if n.wrapping_rem(p) == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || {
                p = (p as std::os::raw::c_uint).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                    as uint as uint;
                n.wrapping_rem(p) == 0 as std::os::raw::c_int as std::os::raw::c_uint
            }
        {
            return 0 as std::os::raw::c_int;
        }
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn reverse(mut n: uint) -> uint {
    let mut r: uint = 0;
    r = 0 as std::os::raw::c_int as uint;
    while n != 0 {
        r = r
            .wrapping_mul(10 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_add(n.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_uint));
        n = (n as std::os::raw::c_uint).wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_uint) as uint
            as uint;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn is_emirp(mut n: uint) -> std::os::raw::c_int {
    let mut r: uint = reverse(n);
    return (r != n && is_prime(n) != 0 && is_prime(r) != 0) as std::os::raw::c_int;
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut x: uint = 0;
    let mut c: uint = 0 as std::os::raw::c_int as uint;
    match argc {
        1 => {
            x = 11 as std::os::raw::c_int as uint;
            while c < 20 as std::os::raw::c_int as std::os::raw::c_uint {
                if is_emirp(x) != 0 {
                    printf(b" %u\0" as *const u8 as *const std::os::raw::c_char, x);
                    c = c.wrapping_add(1);
                    c;
                }
                x = (x as std::os::raw::c_uint).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                    as uint as uint;
            }
        }
        2 => {
            x = 7701 as std::os::raw::c_int as uint;
            while x < 8000 as std::os::raw::c_int as std::os::raw::c_uint {
                if is_emirp(x) != 0 {
                    printf(b" %u\0" as *const u8 as *const std::os::raw::c_char, x);
                }
                x = (x as std::os::raw::c_uint).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                    as uint as uint;
            }
        }
        _ => {
            x = 11 as std::os::raw::c_int as uint;
            loop {
                if is_emirp(x) != 0
                    && {
                        c = c.wrapping_add(1);
                        c == 10000 as std::os::raw::c_int as std::os::raw::c_uint
                    }
                {
                    printf(b"%u\0" as *const u8 as *const std::os::raw::c_char, x);
                    break;
                } else {
                    x = (x as std::os::raw::c_uint)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as uint as uint;
                }
            }
        }
    }
    putchar('\n' as i32);
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

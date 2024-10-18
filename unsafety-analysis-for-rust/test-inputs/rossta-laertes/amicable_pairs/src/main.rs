#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn atoi(__nptr: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
}
pub type uint = std::os::raw::c_uint;
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut top: uint = atoi(*argv.offset(1 as std::os::raw::c_int as isize)) as uint;
    let mut divsum: *mut uint = malloc(
        (top.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<uint>() as std::os::raw::c_ulong),
    ) as *mut uint;
    let mut pows: [uint; 32] = [
        1 as std::os::raw::c_int as uint,
        0 as std::os::raw::c_int as uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: uint = 0 as std::os::raw::c_int as uint;
    while i <= top {
        *divsum.offset(i as isize) = 1 as std::os::raw::c_int as uint;
        i = i.wrapping_add(1);
        i;
    }
    let mut p: uint = 2 as std::os::raw::c_int as uint;
    while p.wrapping_add(p) <= top {
        if *divsum.offset(p as isize) > 1 as std::os::raw::c_int as std::os::raw::c_uint {
            let ref mut fresh0 = *divsum.offset(p as isize);
            *fresh0 = (*fresh0 as std::os::raw::c_uint).wrapping_sub(p) as uint as uint;
        } else {
            let mut x: uint = 0;
            x = 1 as std::os::raw::c_int as uint;
            while pows[x.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as usize]
                <= top.wrapping_div(p)
            {
                pows[x
                    as usize] = p
                    .wrapping_mul(
                        pows[x.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as usize],
                    );
                x = x.wrapping_add(1);
                x;
            }
            let mut k: uint = p.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
            let mut n: uint = p.wrapping_add(p);
            while n <= top {
                let mut s: uint = (1 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(pows[1 as std::os::raw::c_int as usize]);
                k = k.wrapping_sub(1);
                k;
                if k == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    let mut i_0: uint = 2 as std::os::raw::c_int as uint;
                    while i_0 < x && n.wrapping_rem(pows[i_0 as usize]) == 0 {
                        let fresh1 = i_0;
                        i_0 = i_0.wrapping_add(1);
                        s = (s as std::os::raw::c_uint).wrapping_add(pows[fresh1 as usize])
                            as uint as uint;
                    }
                    k = p;
                }
                let ref mut fresh2 = *divsum.offset(n as isize);
                *fresh2 = (*fresh2 as std::os::raw::c_uint).wrapping_mul(s) as uint as uint;
                n = (n as std::os::raw::c_uint).wrapping_add(p) as uint as uint;
            }
        }
        p = p.wrapping_add(1);
        p;
    }
    let mut p_0: uint = (top >> 1 as std::os::raw::c_int)
        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
    while p_0 <= top {
        if *divsum.offset(p_0 as isize) > 1 as std::os::raw::c_int as std::os::raw::c_uint {
            let ref mut fresh3 = *divsum.offset(p_0 as isize);
            *fresh3 = (*fresh3 as std::os::raw::c_uint).wrapping_sub(p_0) as uint as uint;
        }
        p_0 = p_0.wrapping_add(1);
        p_0;
    }
    let mut cnt: uint = 0 as std::os::raw::c_int as uint;
    let mut a: uint = 1 as std::os::raw::c_int as uint;
    while a <= top {
        let mut b: uint = *divsum.offset(a as isize);
        if b > a && b <= top && *divsum.offset(b as isize) == a {
            printf(b"%u %u\n\0" as *const u8 as *const std::os::raw::c_char, a, b);
            cnt = cnt.wrapping_add(1);
            cnt;
        }
        a = a.wrapping_add(1);
        a;
    }
    printf(b"\nTop %u count : %u\n\0" as *const u8 as *const std::os::raw::c_char, top, cnt);
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

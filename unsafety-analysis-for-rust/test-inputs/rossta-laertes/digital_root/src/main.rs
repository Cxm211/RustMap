#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn droot<'a1>(
    mut x: std::os::raw::c_longlong,
    mut base: std::os::raw::c_int,
    mut pers: Option<&'a1 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut d: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if !borrow(& pers).is_none() {
        *borrow_mut(&mut pers).unwrap() = 0 as std::os::raw::c_int;
        while x >= base as std::os::raw::c_longlong {
            d = 0 as std::os::raw::c_int;
            while x != 0 {
                d = (d as std::os::raw::c_longlong + x % base as std::os::raw::c_longlong)
                    as std::os::raw::c_int;
                x /= base as std::os::raw::c_longlong;
            }
            x = d as std::os::raw::c_longlong;
            *borrow_mut(&mut pers).unwrap() += 1;
            *borrow(& pers).unwrap();
        }
    } else if x != 0
        && {
            d = (x % (base - 1 as std::os::raw::c_int) as std::os::raw::c_longlong) as std::os::raw::c_int;
            d == 0
        }
    {
        d = base - 1 as std::os::raw::c_int;
    }
    return d;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut d: std::os::raw::c_int = 0;
    let mut pers: std::os::raw::c_int = 0;
    let mut x: [std::os::raw::c_longlong; 4] = [
        627615 as std::os::raw::c_int as std::os::raw::c_longlong,
        39390 as std::os::raw::c_int as std::os::raw::c_longlong,
        588225 as std::os::raw::c_int as std::os::raw::c_longlong,
        393900588225 as std::os::raw::c_longlong,
    ];
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        d = droot(x[i as usize], 10 as std::os::raw::c_int, Some(&mut pers));
        printf(
            b"%lld: pers %d, root %d\n\0" as *const u8 as *const std::os::raw::c_char,
            x[i as usize],
            pers,
            d,
        );
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

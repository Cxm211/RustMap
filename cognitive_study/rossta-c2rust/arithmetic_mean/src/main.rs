#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mean(
    mut v: *mut std::os::raw::c_double,
    mut len: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < len {
        sum += *v.offset(i as isize);
        i += 1;
        i;
    }
    return sum / len as std::os::raw::c_double;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut v: [std::os::raw::c_double; 5] = [
        1 as std::os::raw::c_int as std::os::raw::c_double,
        2 as std::os::raw::c_int as std::os::raw::c_double,
        2.718f64,
        3 as std::os::raw::c_int as std::os::raw::c_double,
        3.142f64,
    ];
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    len = 5 as std::os::raw::c_int;
    while len >= 0 as std::os::raw::c_int {
        printf(b"mean[\0" as *const u8 as *const std::os::raw::c_char);
        i = 0 as std::os::raw::c_int;
        while i < len {
            printf(
                if i != 0 {
                    b", %g\0" as *const u8 as *const std::os::raw::c_char
                } else {
                    b"%g\0" as *const u8 as *const std::os::raw::c_char
                },
                v[i as usize],
            );
            i += 1;
            i;
        }
        printf(
            b"] = %g\n\0" as *const u8 as *const std::os::raw::c_char,
            mean(v.as_mut_ptr(), len),
        );
        len -= 1;
        len;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

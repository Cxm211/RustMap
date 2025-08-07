#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn repeat(
    mut f: Option::<unsafe extern "C" fn() -> ()>,
    mut n: std::os::raw::c_uint,
) {
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 > 0 as std::os::raw::c_int as std::os::raw::c_uint) {
            break;
        }
        (Some(f.expect("non-null function pointer")))
            .expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn example() {
    printf(b"Example\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    repeat(
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::core::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(example),
            ),
        ),
        4 as std::os::raw::c_int as std::os::raw::c_uint,
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

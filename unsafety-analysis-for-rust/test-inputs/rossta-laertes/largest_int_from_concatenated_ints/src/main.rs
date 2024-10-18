#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn putchar(__c: std::os::raw::c_int) -> std::os::raw::c_int;
    fn qsort(
        __base: *mut u8,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __compar_fn_t = Option<unsafe extern "C"  fn(_: * const u8,_: * const u8,) -> std::os::raw::c_int>;
#[no_mangle]
pub unsafe extern "C" fn catcmp(
    mut a: *const u8,
    mut b: *const u8,
) -> std::os::raw::c_int {
    let mut ab: [std::os::raw::c_char; 32] = [0; 32];
    let mut ba: [std::os::raw::c_char; 32] = [0; 32];
    sprintf(
        ab.as_mut_ptr(),
        b"%d%d\0" as *const u8 as *const std::os::raw::c_char,
        *(a as *mut std::os::raw::c_int),
        *(b as *mut std::os::raw::c_int),
    );
    sprintf(
        ba.as_mut_ptr(),
        b"%d%d\0" as *const u8 as *const std::os::raw::c_char,
        *(b as *mut std::os::raw::c_int),
        *(a as *mut std::os::raw::c_int),
    );
    return strcmp(ba.as_mut_ptr(), ab.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn maxcat(mut a: *mut std::os::raw::c_int, mut len: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    qsort(
        a as *mut u8,
        len as size_t,
        std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
        Some(
            catcmp
                as unsafe extern "C" fn(
                    *const u8,
                    *const u8,
                ) -> std::os::raw::c_int,
        ),
    );
    i = 0 as std::os::raw::c_int;
    while i < len {
        printf(b"%d\0" as *const u8 as *const std::os::raw::c_char, *a.offset(i as isize));
        i += 1;
        i;
    }
    putchar('\n' as i32);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: [std::os::raw::c_int; 8] = [
        1 as std::os::raw::c_int,
        34 as std::os::raw::c_int,
        3 as std::os::raw::c_int,
        98 as std::os::raw::c_int,
        9 as std::os::raw::c_int,
        76 as std::os::raw::c_int,
        45 as std::os::raw::c_int,
        4 as std::os::raw::c_int,
    ];
    let mut y: [std::os::raw::c_int; 4] = [
        54 as std::os::raw::c_int,
        546 as std::os::raw::c_int,
        548 as std::os::raw::c_int,
        60 as std::os::raw::c_int,
    ];
    maxcat(
        x.as_mut_ptr(),
        (std::mem::size_of::<[std::os::raw::c_int; 8]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
    );
    maxcat(
        y.as_mut_ptr(),
        (std::mem::size_of::<[std::os::raw::c_int; 4]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            as std::os::raw::c_int,
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

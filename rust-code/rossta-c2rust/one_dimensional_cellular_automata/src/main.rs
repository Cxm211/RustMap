#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}
#[no_mangle]
pub static mut trans: [std::os::raw::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &mut [std::os::raw::c_char; 9]>(b"___#_##_\0")
};
#[no_mangle]
pub unsafe extern "C" fn evolve(
    mut cell: *mut std::os::raw::c_char,
    mut backup: *mut std::os::raw::c_char,
    mut len: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut diff: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < len {
        *backup
            .offset(
                i as isize,
            ) = trans[((*cell.offset((i - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
            != '_' as i32) as std::os::raw::c_int * 4 as std::os::raw::c_int
            + (*cell.offset(i as isize) as std::os::raw::c_int != '_' as i32) as std::os::raw::c_int
                * 2 as std::os::raw::c_int
            + (*cell.offset((i + 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                != '_' as i32) as std::os::raw::c_int) as usize];
        diff
            += (*backup.offset(i as isize) as std::os::raw::c_int
                != *cell.offset(i as isize) as std::os::raw::c_int) as std::os::raw::c_int;
        i += 1;
        i;
    }
    strcpy(cell, backup as *const std::os::raw::c_char);
    return diff;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut c: [std::os::raw::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [std::os::raw::c_char; 22],
    >(b"_###_##_#_#_#_#__#__\n\0");
    let mut b: [std::os::raw::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [std::os::raw::c_char; 22],
    >(b"____________________\n\0");
    loop {
        printf(c.as_mut_ptr().offset(1 as std::os::raw::c_int as isize));
        if !(evolve(
            c.as_mut_ptr().offset(1 as std::os::raw::c_int as isize),
            b.as_mut_ptr().offset(1 as std::os::raw::c_int as isize),
            (::core::mem::size_of::<[std::os::raw::c_char; 22]>() as std::os::raw::c_ulong)
                .wrapping_sub(3 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int,
        ) != 0)
        {
            break;
        }
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

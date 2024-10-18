#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strncpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_char;
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn move_to_front(
    mut str: *mut std::os::raw::c_char,
    mut c: std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut q: *mut std::os::raw::c_char = core::ptr::null_mut();
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut shift: std::os::raw::c_int = 0 as std::os::raw::c_int;
    p = malloc((strlen(str)).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong))
        as *mut std::os::raw::c_char;
    strcpy(p, str);
    q = strchr(p, c as std::os::raw::c_int);
    shift = q.offset_from(p) as std::os::raw::c_long as std::os::raw::c_int;
    strncpy(str.offset(1 as std::os::raw::c_int as isize), p, shift as std::os::raw::c_ulong);
    *str.offset(0 as std::os::raw::c_int as isize) = c;
    free(p as *mut u8);
    return shift;
}
#[no_mangle]
pub unsafe extern "C" fn decode(
    mut pass: *mut std::os::raw::c_int,
    mut size: std::os::raw::c_int,
    mut sym: *mut std::os::raw::c_char,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut index: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_char = 0;
    let mut table: [std::os::raw::c_char; 27] = *std::mem::transmute::<
        &[u8; 27],
        &mut [std::os::raw::c_char; 27],
    >(b"abcdefghijklmnopqrstuvwxyz\0");
    i = 0 as std::os::raw::c_int;
    while i < size {
        c = table[*pass.offset(i as isize) as usize];
        index = move_to_front(table.as_mut_ptr(), c);
        if *pass.offset(i as isize) != index {
            printf(b"there is an error\0" as *const u8 as *const std::os::raw::c_char);
        }
        *sym.offset(i as isize) = c;
        i += 1;
        i;
    }
    *sym.offset(size as isize) = '\0' as i32 as std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn encode(
    mut sym: *mut std::os::raw::c_char,
    mut size: std::os::raw::c_int,
    mut pass: *mut std::os::raw::c_int,
) {
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut c: std::os::raw::c_char = 0;
    let mut table: [std::os::raw::c_char; 27] = *std::mem::transmute::<
        &[u8; 27],
        &mut [std::os::raw::c_char; 27],
    >(b"abcdefghijklmnopqrstuvwxyz\0");
    i = 0 as std::os::raw::c_int;
    while i < size {
        c = *sym.offset(i as isize);
        *pass.offset(i as isize) = move_to_front(table.as_mut_ptr(), c);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn check(
    mut sym: *mut std::os::raw::c_char,
    mut size: std::os::raw::c_int,
    mut pass: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut pass2: *mut std::os::raw::c_int = malloc(
        (std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(size as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let mut sym2: *mut std::os::raw::c_char = malloc(
        (std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong)
            .wrapping_mul(size as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    let mut val: std::os::raw::c_int = 1 as std::os::raw::c_int;
    encode(sym, size, pass2);
    i = 0 as std::os::raw::c_int;
    while i < size && *pass.offset(i as isize) == *pass2.offset(i as isize) {
        i += 1;
        i;
    }
    if i != size {
        val = 0 as std::os::raw::c_int;
    }
    decode(pass, size, sym2);
    if strcmp(sym, sym2) != 0 as std::os::raw::c_int {
        val = 0 as std::os::raw::c_int;
    }
    free(sym2 as *mut u8);
    free(pass2 as *mut u8);
    return val;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut sym: [[std::os::raw::c_char; 100]; 3] = [
        *std::mem::transmute::<
            &[u8; 100],
            &mut [std::os::raw::c_char; 100],
        >(
            b"broood\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *std::mem::transmute::<
            &[u8; 100],
            &mut [std::os::raw::c_char; 100],
        >(
            b"bananaaa\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
        *std::mem::transmute::<
            &[u8; 100],
            &mut [std::os::raw::c_char; 100],
        >(
            b"hiphophiphop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ),
    ];
    let mut pass: [std::os::raw::c_int; 100] = [
        0 as std::os::raw::c_int,
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
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 3 as std::os::raw::c_int {
        len = strlen((sym[i as usize]).as_mut_ptr()) as std::os::raw::c_int;
        encode((sym[i as usize]).as_mut_ptr(), len, pass.as_mut_ptr());
        printf(
            b"%s : [\0" as *const u8 as *const std::os::raw::c_char,
            (sym[i as usize]).as_mut_ptr(),
        );
        j = 0 as std::os::raw::c_int;
        while j < len {
            printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, pass[j as usize]);
            j += 1;
            j;
        }
        printf(b"]\n\0" as *const u8 as *const std::os::raw::c_char);
        if check((sym[i as usize]).as_mut_ptr(), len, pass.as_mut_ptr()) != 0 {
            printf(b"Correct :)\n\0" as *const u8 as *const std::os::raw::c_char);
        } else {
            printf(b"Incorrect :(\n\0" as *const u8 as *const std::os::raw::c_char);
        }
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

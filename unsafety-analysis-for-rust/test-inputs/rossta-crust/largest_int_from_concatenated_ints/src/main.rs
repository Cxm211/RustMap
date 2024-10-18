#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
//use c2rust_out::*;
extern "C" {
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[no_mangle]
pub extern "C" fn catcmp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    unsafe {
        let mut ab: [i8; 32] = [0; 32];
        let mut ba: [i8; 32] = [0; 32];
        sprintf(
            ab.as_mut_ptr(),
            b"%d%d\0" as *const u8 as *const i8,
            *(a as *mut i32),
            *(b as *mut i32),
        );
        sprintf(
            ba.as_mut_ptr(),
            b"%d%d\0" as *const u8 as *const i8,
            *(b as *mut i32),
            *(a as *mut i32),
        );
        return strcmp(ba.as_mut_ptr(), ab.as_mut_ptr());
    }
}

#[no_mangle]
pub extern "C" fn maxcat(mut a: *mut i32, mut len: i32) {
    unsafe {
        let mut i: i32 = 0;
        qsort(
            a as *mut libc::c_void,
            len as u64,
            ::core::mem::size_of::<i32>() as u64,
            Some(catcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        i = 0;
        while i < len {
            print!("{}", *a.offset(i as isize));
            i += 1;
            i;
        }
        print!("{}", '\n' as i32);
    }
}

fn main_0() -> i32 {
    let mut x: [i32; 8] = [1, 34, 3, 98, 9, 76, 45, 4];
    let mut y: [i32; 4] = [54, 546, 548, 60];
    maxcat(
        x.as_mut_ptr(),
        (::core::mem::size_of::<[i32; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
    );
    maxcat(
        y.as_mut_ptr(),
        (::core::mem::size_of::<[i32; 4]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn rand() -> std::os::raw::c_int;
    fn qsort(
        __base: *mut u8,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = std::os::raw::c_ulong;
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
pub type __compar_fn_t = Option<unsafe extern "C"  fn(_: * const u8,_: * const u8,) -> std::os::raw::c_int>;
#[no_mangle]
pub unsafe extern "C" fn compareInts(
    mut i1: *const u8,
    mut i2: *const u8,
) -> std::os::raw::c_int {
    let mut a: std::os::raw::c_int = *(i1 as *mut std::os::raw::c_int);
    let mut b: std::os::raw::c_int = *(i2 as *mut std::os::raw::c_int);
    return a - b;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut nsum: std::os::raw::c_int = 0;
    let mut vsum: std::os::raw::c_int = 0;
    let mut vcount: std::os::raw::c_int = 0;
    let mut values: [std::os::raw::c_int; 6] = [0; 6];
    let mut numbers: [std::os::raw::c_int; 4] = [0; 4];
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    loop {
        vsum = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < 6 as std::os::raw::c_int {
            j = 0 as std::os::raw::c_int;
            while j < 4 as std::os::raw::c_int {
                numbers[j as usize] = 1 as std::os::raw::c_int + rand() % 6 as std::os::raw::c_int;
                j += 1;
                j;
            }
            qsort(
                numbers.as_mut_ptr() as *mut u8,
                4 as std::os::raw::c_int as size_t,
                std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong,
                Some(
                    compareInts
                        as unsafe extern "C" fn(
                            *const u8,
                            *const u8,
                        ) -> std::os::raw::c_int,
                ),
            );
            nsum = 0 as std::os::raw::c_int;
            j = 1 as std::os::raw::c_int;
            while j < 4 as std::os::raw::c_int {
                nsum += numbers[j as usize];
                j += 1;
                j;
            }
            values[i as usize] = nsum;
            vsum += values[i as usize];
            i += 1;
            i;
        }
        if vsum < 75 as std::os::raw::c_int {
            continue;
        }
        vcount = 0 as std::os::raw::c_int;
        j = 0 as std::os::raw::c_int;
        while j < 6 as std::os::raw::c_int {
            if values[j as usize] >= 15 as std::os::raw::c_int {
                vcount += 1;
                vcount;
            }
            j += 1;
            j;
        }
        if vcount < 2 as std::os::raw::c_int {
            continue;
        }
        printf(
            b"The 6 random numbers generated are:\n\0" as *const u8
                as *const std::os::raw::c_char,
        );
        printf(b"[\0" as *const u8 as *const std::os::raw::c_char);
        j = 0 as std::os::raw::c_int;
        while j < 6 as std::os::raw::c_int {
            printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, values[j as usize]);
            j += 1;
            j;
        }
        printf(b"\x08]\n\0" as *const u8 as *const std::os::raw::c_char);
        printf(
            b"\nTheir sum is %d and %d of them are >= 15\n\0" as *const u8
                as *const std::os::raw::c_char,
            vsum,
            vcount,
        );
        break;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

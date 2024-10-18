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
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

//use c2rust_out::*;
extern "C" {
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn rprint(mut s: *mut i8, mut x: *mut i32, mut len: i32) -> u64 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut a: *mut i8 = s;
        j = 0;
        i = j;
        while i < len {
            while j < len - 1 && *x.offset((j + 1i32) as isize) == *x.offset(j as isize) + 1 {
                j += 1;
                j;
            }
            if (i + 1) < j {
                a = a.offset(snprintf(
                    if !s.is_null() { a } else { s },
                    (if !s.is_null() { 100 } else { 0 }) as u64,
                    b"%s%d-%d\0" as *const u8 as *const i8,
                    if a > s {
                        b",\0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                    *x.offset(i as isize),
                    *x.offset(j as isize),
                ) as isize);
            } else {
                while i <= j {
                    let fresh0 = i;
                    i = i + 1;
                    a = a.offset(snprintf(
                        if !s.is_null() { a } else { s },
                        (if !s.is_null() { 100 } else { 0 }) as u64,
                        b"%s%d\0" as *const u8 as *const i8,
                        if a > s {
                            b",\0" as *const u8 as *const i8
                        } else {
                            b"\0" as *const u8 as *const i8
                        },
                        *x.offset(fresh0 as isize),
                    ) as isize);
                }
            }
            j += 1;
            i = j;
        }
        return a.offset_from(s) as u64;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut x: [i32; 33] = [
            0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28,
            29, 30, 31, 32, 33, 35, 36, 37, 38, 39,
        ];
        let mut s: *mut i8 = malloc(
            (rprint(
                0 as *mut i8,
                x.as_mut_ptr(),
                (::core::mem::size_of::<[i32; 33]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
            ))
            .wrapping_add(1),
        ) as *mut i8;
        rprint(
            s,
            x.as_mut_ptr(),
            (::core::mem::size_of::<[i32; 33]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
        );
        print!("{}\n", build_str_from_raw_ptr(s as *mut u8));
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

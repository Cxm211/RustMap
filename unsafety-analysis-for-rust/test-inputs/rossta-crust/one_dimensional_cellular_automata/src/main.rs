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
    fn printf(_: *const i8, _: ...) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
}
#[no_mangle]
pub static mut trans: [i8; 9] =
    unsafe { *::core::mem::transmute::<&[u8; 9], &mut [i8; 9]>(b"___#_##_\0") };
#[no_mangle]
pub extern "C" fn evolve(mut cell: *mut i8, mut backup: *mut i8, mut len: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut diff: i32 = 0;
        i = 0;
        while i < len {
            *backup.offset(i as isize) =
                trans[((*cell.offset((i - 1i32) as isize) as i32 != '_' as i32) as i32 * 4
                    + (*cell.offset(i as isize) as i32 != '_' as i32) as i32 * 2
                    + (*cell.offset((i + 1i32) as isize) as i32 != '_' as i32) as i32)
                    as usize];
            diff += (*backup.offset(i as isize) as i32 != *cell.offset(i as isize) as i32) as i32;
            i += 1;
            i;
        }
        strcpy(cell, backup as *const i8);
        return diff;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut c: [i8; 22] =
            *::core::mem::transmute::<&[u8; 22], &mut [i8; 22]>(b"_###_##_#_#_#_#__#__\n\0");
        let mut b: [i8; 22] =
            *::core::mem::transmute::<&[u8; 22], &mut [i8; 22]>(b"____________________\n\0");
        loop {
            printf(c.as_mut_ptr().offset(1 as isize));
            if !(evolve(
                c.as_mut_ptr().offset(1 as isize),
                b.as_mut_ptr().offset(1 as isize),
                (::core::mem::size_of::<[i8; 22]>() as u64).wrapping_sub(3) as i32,
            ) != 0)
            {
                break;
            }
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn hailstone(mut n: i32, mut arry: *mut i32) -> i32 {
    unsafe {
        let mut hs: i32 = 1;
        while n != 1 {
            hs += 1;
            hs;
            if !arry.is_null() {
                let fresh0 = arry;
                arry = arry.offset(1);
                *fresh0 = n;
            }
            n = if n & 1 != 0 { 3 * n + 1 } else { n / 2 };
        }
        if !arry.is_null() {
            let fresh1 = arry;
            arry = arry.offset(1);
            *fresh1 = n;
        }
        return hs;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut j: i32 = 0;
        let mut hmax: i32 = 0;
        let mut jatmax: i32 = 0;
        let mut n: i32 = 0;
        let mut arry: *mut i32 = 0 as *mut i32;
        j = 1;
        while j < 100000 {
            n = hailstone(j, 0 as *mut i32);
            if hmax < n {
                hmax = n;
                jatmax = j;
            }
            j += 1;
            j;
        }
        n = hailstone(27, 0 as *mut i32);
        arry = malloc((n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)) as *mut i32;
        n = hailstone(27, arry);
        print!(
            "[ {}, {}, {}, {}, ...., {}, {}, {}, {}] len={}\n",
            *arry.offset(0 as isize),
            *arry.offset(1 as isize),
            *arry.offset(2 as isize),
            *arry.offset(3 as isize),
            *arry.offset((n - 4i32) as isize),
            *arry.offset((n - 3i32) as isize),
            *arry.offset((n - 2i32) as isize),
            *arry.offset((n - 1i32) as isize),
            n
        );
        print!("Max {} at j= {}\n", hmax, jatmax);
        free(arry as *mut libc::c_void);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

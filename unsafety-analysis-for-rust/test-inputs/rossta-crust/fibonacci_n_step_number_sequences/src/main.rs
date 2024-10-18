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
}
#[no_mangle]
pub extern "C" fn anynacci(mut seedArray: *mut i32, mut howMany: i32) -> *mut i32 {
    unsafe {
        let mut result: *mut i32 =
            malloc((howMany as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)) as *mut i32;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut initialCardinality: i32 = 0;
        i = 0;
        while *seedArray.offset(i as isize) != 0 {
            i += 1;
            i;
        }
        initialCardinality = i;
        i = 0;
        while i < initialCardinality {
            *result.offset(i as isize) = *seedArray.offset(i as isize);
            i += 1;
            i;
        }
        i = initialCardinality;
        while i < howMany {
            *result.offset(i as isize) = 0;
            j = i - initialCardinality;
            while j < i {
                *result.offset(i as isize) += *result.offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        return result;
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut fibo: [i32; 3] = [1, 1, 0];
        let mut tribo: [i32; 4] = [1, 1, 2, 0];
        let mut tetra: [i32; 5] = [1, 1, 2, 4, 0];
        let mut luca: [i32; 3] = [2, 1, 0];
        let mut fibonacci: *mut i32 = anynacci(fibo.as_mut_ptr(), 10);
        let mut tribonacci: *mut i32 = anynacci(tribo.as_mut_ptr(), 10);
        let mut tetranacci: *mut i32 = anynacci(tetra.as_mut_ptr(), 10);
        let mut lucas: *mut i32 = anynacci(luca.as_mut_ptr(), 10);
        let mut i: i32 = 0;
        print!("\nFibonacci	Tribonacci	Tetranacci	Lucas\n");
        i = 0;
        while i < 10 {
            print!(
                "\n{}		{}		{}		{}",
                *fibonacci.offset(i as isize),
                *tribonacci.offset(i as isize),
                *tetranacci.offset(i as isize),
                *lucas.offset(i as isize)
            );
            i += 1;
            i;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
extern "C" {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub start: i32,
    pub end: i32,
    pub sum: i32,
}
#[no_mangle]
pub extern "C" fn maxSubseq(mut sequence: *const i32, len: i32) -> Range {
    unsafe {
        let mut maxSum: i32 = 0;
        let mut thisSum: i32 = 0;
        let mut i: i32 = 0;
        let mut start: i32 = 0;
        let mut end: i32 = -1;
        let mut j: i32 = 0;
        j = 0;
        while j < len {
            thisSum += *sequence.offset(j as isize);
            if thisSum < 0 {
                i = j + 1;
                thisSum = 0;
            } else if thisSum > maxSum {
                maxSum = thisSum;
                start = i;
                end = j;
            }
            j += 1;
            j;
        }
        let mut r: Range = Range {
            start: 0,
            end: 0,
            sum: 0,
        };
        if start <= end && start >= 0 && end >= 0 {
            r.start = start;
            r.end = end + 1;
            r.sum = maxSum;
        } else {
            r.start = 0;
            r.end = 0;
            r.sum = 0;
        }
        return r;
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut a: [i32; 11] = [-1, -2, 3, 5, 6, -2, -1, 4, -4, 2, -1];
        let mut alength: i32 = (::core::mem::size_of::<[i32; 11]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            as i32;
        let mut r: Range = maxSubseq(a.as_mut_ptr() as *const i32, alength);
        print!("Max sum = {}\n", r.sum);
        let mut i: i32 = 0;
        i = r.start;
        while i < r.end {
            print!("{} ", a[i as usize]);
            i += 1;
            i;
        }
        print!("\n");
        return 0;
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}

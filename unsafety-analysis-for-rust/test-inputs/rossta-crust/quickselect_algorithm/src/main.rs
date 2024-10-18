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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn qselect(mut v: *mut i32, mut len: i32, mut k: i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut st: i32 = 0;
        let mut tmp: i32 = 0;
        i = 0;
        st = i;
        while i < len - 1 {
            if !(*v.offset(i as isize) > *v.offset((len - 1i32) as isize)) {
                tmp = *v.offset(i as isize);
                *v.offset(i as isize) = *v.offset(st as isize);
                *v.offset(st as isize) = tmp;
                st += 1;
                st;
            }
            i += 1;
            i;
        }
        tmp = *v.offset((len - 1i32) as isize);
        *v.offset((len - 1i32) as isize) = *v.offset(st as isize);
        *v.offset(st as isize) = tmp;
        return if k == st {
            *v.offset(st as isize)
        } else if st > k {
            qselect(v, st, k)
        } else {
            qselect(v.offset(st as isize), len - st, k - st)
        };
    }
}

fn main_0() -> i32 {
    let mut x: [i32; 10] = [9, 8, 7, 6, 5, 0, 1, 2, 3, 4];
    let mut y: [i32; 10] = [0; 10];
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while i < 10 {
            memcpy(
                y.as_mut_ptr() as *mut libc::c_void,
                x.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[i32; 10]>() as u64,
            );
            print!("{}: {}\n", i, qselect(y.as_mut_ptr(), 10, i));
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

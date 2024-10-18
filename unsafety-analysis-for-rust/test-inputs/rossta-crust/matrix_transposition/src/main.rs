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
#[no_mangle]
pub extern "C" fn transpose(
    mut dest: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut src_h: i32,
    mut src_w: i32,
) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let vla = src_h as usize;
        let mut d: *mut f64 = dest as *mut f64;
        let vla_0 = src_w as usize;
        let mut s: *mut f64 = src as *mut f64;
        i = 0;
        while i < src_h {
            j = 0;
            while j < src_w {
                *d.offset(j as isize * vla as isize).offset(i as isize) =
                    *s.offset(i as isize * vla_0 as isize).offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut a: [[f64; 5]; 3] = [
        [0 as f64, 1 as f64, 2 as f64, 3 as f64, 4 as f64],
        [5 as f64, 6 as f64, 7 as f64, 8 as f64, 9 as f64],
        [1 as f64, 0 as f64, 0 as f64, 0 as f64, 42 as f64],
    ];
    let mut b: [[f64; 3]; 5] = [[0.; 3]; 5];
    transpose(
        b.as_mut_ptr() as *mut libc::c_void,
        a.as_mut_ptr() as *mut libc::c_void,
        3,
        5,
    );
    i = 0;
    while i < 5 {
        j = 0;
        while j < 3 {
            if j == 2 {
                print!("{}{}", b[i as usize][j as usize], '\n' as i32)
            } else {
                print!("{}{}", b[i as usize][j as usize], ' ' as i32)
            };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

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
fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut flip: i32 = 0;
        let mut q: *mut i32 = (malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(100000))
            as *mut i32)
            .offset(-(1 as isize));
        let ref mut fresh0 = *q.offset(2 as isize);
        *fresh0 = 1;
        *q.offset(1 as isize) = *fresh0;
        i = 3;
        while i <= 100000 {
            *q.offset(i as isize) = *q.offset((i - *q.offset((i - 1i32) as isize)) as isize)
                + *q.offset((i - *q.offset((i - 2i32) as isize)) as isize);
            i += 1;
            i;
        }
        i = 1;
        while i <= 10 {
            if i == 10 {
                print!("{}{}", *q.offset(i as isize), '\n' as i32)
            } else {
                print!("{}{}", *q.offset(i as isize), ' ' as i32)
            };
            i += 1;
            i;
        }
        print!("{}\n", *q.offset(1000 as isize));
        flip = 0;
        i = 1;
        while i < 100000 {
            flip += (*q.offset(i as isize) > *q.offset((i + 1i32) as isize)) as i32;
            i += 1;
            i;
        }
        print!("flips: {}\n", flip);
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

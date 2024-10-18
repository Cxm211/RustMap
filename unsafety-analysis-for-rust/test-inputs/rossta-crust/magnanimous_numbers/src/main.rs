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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
}
#[no_mangle]
pub extern "C" fn is_prime(mut n: u64) -> i32 {
    let mut d: u64 = 0;
    if n < 2 {
        return 0;
    }
    if n.wrapping_rem(2) == 0 {
        return (n == 2) as i32;
    }
    if n.wrapping_rem(3) == 0 {
        return (n == 3) as i32;
    }
    d = 5;
    while d.wrapping_mul(d) <= n {
        if n.wrapping_rem(d) == 0 {
            return 0;
        }
        d = (d).wrapping_add(2) as u64;
        if n.wrapping_rem(d) == 0 {
            return 0;
        }
        d = (d).wrapping_add(4) as u64;
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn ord(mut res: *mut i8, mut n: i32) {
    unsafe {
        let mut suffix: [i8; 3] = [0; 3];
        let mut m: i32 = n % 100;
        if m >= 4 && m <= 20 {
            sprintf(res, b"%dth\0" as *const u8 as *const i8, n);
            return;
        }
        match m % 10 {
            1 => {
                strcpy(suffix.as_mut_ptr(), b"st\0" as *const u8 as *const i8);
            }
            2 => {
                strcpy(suffix.as_mut_ptr(), b"nd\0" as *const u8 as *const i8);
            }
            3 => {
                strcpy(suffix.as_mut_ptr(), b"rd\0" as *const u8 as *const i8);
            }
            _ => {
                strcpy(suffix.as_mut_ptr(), b"th\0" as *const u8 as *const i8);
            }
        }
        sprintf(
            res,
            b"%d%s\0" as *const u8 as *const i8,
            n,
            suffix.as_mut_ptr(),
        );
    }
}

#[no_mangle]
pub extern "C" fn is_magnanimous(mut n: u64) -> i32 {
    let mut p: u64 = 0;
    let mut q: u64 = 0;
    let mut r: u64 = 0;
    if n < 10 {
        return 1;
    }
    p = 10;
    loop {
        q = n.wrapping_div(p);
        r = n.wrapping_rem(p);
        if is_prime(q.wrapping_add(r)) == 0 {
            return 0;
        }
        if q < 10 {
            break;
        }
        p = (p).wrapping_mul(10) as u64;
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn list_mags(mut from: i32, mut thru: i32, mut digs: i32, mut per_line: i32) {
    let mut i: u64 = 0;
    let mut c: i32 = 0;
    let mut res1: [i8; 13] = [0; 13];
    let mut res2: [i8; 13] = [0; 13];
    unsafe {
        if from < 2 {
            print!("\nFirst {} magnanimous numbers:\n", thru);
        } else {
            ord(res1.as_mut_ptr(), from);
            ord(res2.as_mut_ptr(), thru);
            print!(
                "\n{} through {} magnanimous numbers:\n",
                build_str_from_raw_ptr(res1.as_mut_ptr() as *mut u8),
                build_str_from_raw_ptr(res2.as_mut_ptr() as *mut u8)
            );
        }
    }
    while c < thru {
        if is_magnanimous(i) != 0 {
            c += 1;
            if c >= from {
                print!("{1:0$} ", (digs).abs() as usize, i);
                if c % per_line == 0 {
                    print!("\n");
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}

fn main_0() -> i32 {
    list_mags(1, 45, 3, 15);
    list_mags(241, 250, 1, 10);
    list_mags(391, 400, 1, 10);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

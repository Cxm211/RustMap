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
extern "C" {}
#[no_mangle]
pub extern "C" fn show_set(mut x: u32, mut name: *const i8) {
    unsafe {
        let mut i: i32 = 0;
        print!("{} is:", build_str_from_raw_ptr(name as *mut u8));
        i = 0;
        while 1 << i <= x {
            if x & 1 << i != 0 {
                print!(" {}", i);
            }
            i += 1;
            i;
        }
        print!("{}", '\n' as i32);
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    a = 0;
    i = 0;
    while i < 10 {
        a |= 1 << i;
        i += 3;
    }
    show_set(a, b"a\0" as *const u8 as *const i8);
    i = 0;
    while i < 5 {
        if a & 1 << i != 0 {
            print!("	{}{} in set a\n", i, "\0")
        } else {
            print!("	{}{} in set a\n", i, " not\0")
        };
        i += 1;
        i;
    }
    b = a;
    b |= 1 << 5;
    b |= 1 << 10;
    b &= !(1 << 0);
    show_set(b, b"b\0" as *const u8 as *const i8);
    show_set(a | b, b"union(a, b)\0" as *const u8 as *const i8);
    c = a & b;
    show_set(c, b"c = common(a, b)\0" as *const u8 as *const i8);
    show_set(a & !b, b"a - b\0" as *const u8 as *const i8);
    show_set(b & !a, b"b - a\0" as *const u8 as *const i8);
    if b & !a == 0 {
        print!("b is{} a subset of a\n", "\0")
    } else {
        print!("b is{} a subset of a\n", " not\0")
    };
    if c & !a == 0 {
        print!("c is{} a subset of a\n", "\0")
    } else {
        print!("c is{} a subset of a\n", " not\0")
    };
    if (a | b) & !(a & b) == a & !b | b & !a {
        print!(
            "union(a, b) - common(a, b) {} union(a - b, b - a)\n",
            "equals\0"
        )
    } else {
        print!(
            "union(a, b) - common(a, b) {} union(a - b, b - a)\n",
            "does not equal\0"
        )
    };
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

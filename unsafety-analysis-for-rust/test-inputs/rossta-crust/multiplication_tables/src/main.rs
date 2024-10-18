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
fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 12;
    j = 1;
    while j <= n {
        if j != n {
            print!("{:3}{}", j, ' ' as i32)
        } else {
            print!("{:3}{}", j, '\n' as i32)
        };
        j += 1;
        j;
    }
    j = 0;
    while j <= n {
        if j != n {
            print!("----")
        } else {
            print!("+\n")
        };
        j += 1;
        j;
    }
    i = 1;
    while i <= n {
        j = 1;
        while j <= n {
            if j < i {
                print!("    ")
            } else {
                print!("{:3} ", i * j)
            };
            j += 1;
            j;
        }
        print!("| {}\n", i);
        i += 1;
        i;
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

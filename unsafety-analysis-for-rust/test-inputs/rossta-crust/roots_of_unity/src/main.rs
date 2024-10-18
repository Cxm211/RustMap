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
    fn atan2(_: f64, _: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
}
fn main_0() -> i32 {
    let mut a: f64 = 0.;
    let mut c: f64 = 0.;
    let mut s: f64 = 0.;
    unsafe {
        let mut PI2: f64 = atan2(1 as f64, 1 as f64) * 8 as f64;
        let mut n: i32 = 0;
        let mut i: i32 = 0;
        n = 1;
        while n < 10 {
            i = 0;
            while i < n {
                s = 0 as f64;
                c = s;
                if i == 0 {
                    c = 1 as f64;
                } else if n == 4 * i {
                    s = 1 as f64;
                } else if n == 2 * i {
                    c = -1i32 as f64;
                } else if 3 * n == 4 * i {
                    s = -1i32 as f64;
                } else {
                    a = i as f64 * PI2 / n as f64;
                    c = cos(a);
                    s = sin(a);
                }
                if c != 0. {
                    print!("{:.2}", c);
                }
                if s == 1 as f64 {
                    print!("i")
                } else {
                    if s == -1i32 as f64 {
                        print!("-i")
                    } else {
                        if s != 0. {
                            print!("{:+.2}i", s)
                        } else {
                            print!("")
                        }
                    }
                };
                if i == n - 1 {
                    print!("\n")
                } else {
                    print!(",  ")
                };
                i += 1;
                i;
            }
            n += 1;
            n;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

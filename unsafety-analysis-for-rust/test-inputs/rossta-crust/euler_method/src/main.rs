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
    fn exp(_: f64) -> f64;
}
pub type deriv_f = Option<unsafe extern "C" fn(f64, f64) -> f64>;
#[no_mangle]
pub extern "C" fn ivp_euler(mut f: deriv_f, mut y: f64, mut step: i32, mut end_t: i32) {
    let mut t: i32 = 0;
    print!(" Step {:2}: ", step);
    unsafe {
        loop {
            if t % 10 == 0 {
                print!(" {:7.3}", y);
            }
            y += step as f64 * f.expect("non-null function pointer")(t as f64, y);
            t += step;
            if !(t <= end_t) {
                break;
            }
        }
    }
    print!("\n");
}

#[no_mangle]
pub extern "C" fn analytic() {
    let mut t: f64 = 0.;
    print!("    Time: ");
    t = 0 as f64;
    while t <= 100 as f64 {
        print!(" {:7}", t);
        t += 10 as f64;
    }
    print!("\nAnalytic: ");
    t = 0 as f64;
    unsafe {
        while t <= 100 as f64 {
            print!(" {:7.3}", 20 as f64 + 80 as f64 * exp(-0.07f64 * t));
            t += 10 as f64;
        }
    }
    print!("\n");
}

#[no_mangle]
pub extern "C" fn cooling(mut t: f64, mut temp: f64) -> f64 {
    return -0.07f64 * (temp - 20 as f64);
}

fn main_0() -> i32 {
    analytic();
    ivp_euler(
        Some(cooling as unsafe extern "C" fn(f64, f64) -> f64),
        100 as f64,
        2,
        100,
    );
    ivp_euler(
        Some(cooling as unsafe extern "C" fn(f64, f64) -> f64),
        100 as f64,
        5,
        100,
    );
    ivp_euler(
        Some(cooling as unsafe extern "C" fn(f64, f64) -> f64),
        100 as f64,
        10,
        100,
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

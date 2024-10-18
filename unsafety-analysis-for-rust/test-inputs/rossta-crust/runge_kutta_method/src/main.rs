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
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn rk4(
    mut f: Option<unsafe extern "C" fn(f64, f64) -> f64>,
    mut dx: f64,
    mut x: f64,
    mut y: f64,
) -> f64 {
    unsafe {
        let mut k1: f64 = dx * f.expect("non-null function pointer")(x, y);
        let mut k2: f64 =
            dx * f.expect("non-null function pointer")(x + dx / 2 as f64, y + k1 / 2 as f64);
        let mut k3: f64 =
            dx * f.expect("non-null function pointer")(x + dx / 2 as f64, y + k2 / 2 as f64);
        let mut k4: f64 = dx * f.expect("non-null function pointer")(x + dx, y + k3);
        return y + (k1 + 2 as f64 * k2 + 2 as f64 * k3 + k4) / 6 as f64;
    }
}

#[no_mangle]
pub extern "C" fn rate(mut x: f64, mut y: f64) -> f64 {
    unsafe {
        return x * sqrt(y);
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut y: *mut f64 = 0 as *mut f64;
        let mut x: f64 = 0.;
        let mut y2: f64 = 0.;
        let mut x0: f64 = 0 as f64;
        let mut x1: f64 = 10 as f64;
        let mut dx: f64 = 0.1f64;
        let mut i: i32 = 0;
        let mut n: i32 = (1 as f64 + (x1 - x0) / dx) as i32;
        y = malloc((::core::mem::size_of::<f64>() as u64).wrapping_mul(n as u64)) as *mut f64;
        *y.offset(0 as isize) = 1 as f64;
        i = 1;
        while i < n {
            *y.offset(i as isize) = rk4(
                Some(rate as unsafe extern "C" fn(f64, f64) -> f64),
                dx,
                x0 + dx * (i - 1i32) as f64,
                *y.offset((i - 1i32) as isize),
            );
            i += 1;
            i;
        }
        print!("x	y	rel. err.\n------------\n");
        i = 0;
        while i < n {
            x = x0 + dx * i as f64;
            y2 = pow(x * x / 4 as f64 + 1 as f64, 2 as f64);
            print!(
                "{}	{}	{}\n",
                x,
                *y.offset(i as isize),
                *y.offset(i as isize) / y2 - 1 as f64
            );
            i += 10;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

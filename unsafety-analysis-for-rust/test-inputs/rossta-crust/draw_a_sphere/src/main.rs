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
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn ceil(_: f64) -> f64;
    fn floor(_: f64) -> f64;
}
#[no_mangle]
pub static mut shades: *const i8 = b".:!*oe&#%@\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut light: [f64; 3] = [30 as f64, 30 as f64, -50i32 as f64];
#[no_mangle]
pub extern "C" fn normalize(mut v: *mut f64) {
    unsafe {
        let mut len: f64 = sqrt(
            *v.offset(0 as isize) * *v.offset(0 as isize)
                + *v.offset(1 as isize) * *v.offset(1 as isize)
                + *v.offset(2 as isize) * *v.offset(2 as isize),
        );
        *v.offset(0 as isize) /= len;
        *v.offset(1 as isize) /= len;
        *v.offset(2 as isize) /= len;
    }
}

#[no_mangle]
pub extern "C" fn dot(mut x: *mut f64, mut y: *mut f64) -> f64 {
    unsafe {
        let mut d: f64 = *x.offset(0 as isize) * *y.offset(0 as isize)
            + *x.offset(1 as isize) * *y.offset(1 as isize)
            + *x.offset(2 as isize) * *y.offset(2 as isize);
        return if d < 0 as f64 { -d } else { 0 as f64 };
    }
}

#[no_mangle]
pub extern "C" fn draw_sphere(mut R: f64, mut k: f64, mut ambient: f64) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut intensity: i32 = 0;
    let mut b: f64 = 0.;
    let mut vec: [f64; 3] = [0.; 3];
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    unsafe {
        i = floor(-R) as i32;
        while i as f64 <= ceil(R) {
            x = i as f64 + 0.5f64;
            j = floor(-2i32 as f64 * R) as i32;
            while j as f64 <= ceil(2 as f64 * R) {
                y = j as f64 / 2.0f64 + 0.5f64;
                if x * x + y * y <= R * R {
                    vec[0 as usize] = x;
                    vec[1 as usize] = y;
                    vec[2 as usize] = sqrt(R * R - x * x - y * y);
                    normalize(vec.as_mut_ptr());
                    b = pow(dot(light.as_mut_ptr(), vec.as_mut_ptr()), k) + ambient;
                    intensity = ((1 as f64 - b)
                        * (::core::mem::size_of::<*const i8>() as u64).wrapping_sub(1u64) as f64)
                        as i32;
                    if intensity < 0 {
                        intensity = 0;
                    }
                    if intensity as u64
                        >= (::core::mem::size_of::<*const i8>() as u64).wrapping_sub(1)
                    {
                        intensity =
                            (::core::mem::size_of::<*const i8>() as u64).wrapping_sub(2) as i32;
                    }
                    print!("{}", *shades.offset(intensity as isize) as i32);
                } else {
                    print!("{}", ' ' as i32);
                }
                j += 1;
                j;
            }
            print!("{}", '\n' as i32);
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    unsafe {
        normalize(light.as_mut_ptr());
    }
    draw_sphere(20 as f64, 4 as f64, 0.1f64);
    draw_sphere(10 as f64, 2 as f64, 0.4f64);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

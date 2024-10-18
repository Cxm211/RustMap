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
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn rms(mut v: *mut f64, mut n: i32) -> f64 {
    unsafe {
        let mut i: i32 = 0;
        let mut sum: f64 = 0.0f64;
        i = 0;
        while i < n {
            sum += *v.offset(i as isize) * *v.offset(i as isize);
            i += 1;
            i;
        }
        return sqrt(sum / n as f64);
    }
}

fn main_0() -> i32 {
    let mut v: [f64; 10] = [
        1.0f64, 2.0f64, 3.0f64, 4.0f64, 5.0f64, 6.0f64, 7.0f64, 8.0f64, 9.0f64, 10.0f64,
    ];
    print!(
        "{}\n",
        rms(
            v.as_mut_ptr(),
            (::core::mem::size_of::<[f64; 10]>() as u64)
                .wrapping_div(::core::mem::size_of::<f64>() as u64) as i32,
        )
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}

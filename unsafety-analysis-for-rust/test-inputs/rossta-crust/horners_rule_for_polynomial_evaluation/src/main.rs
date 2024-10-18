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
pub extern "C" fn horner(mut coeffs: *mut f64, mut s: i32, mut x: f64) -> f64 {
    unsafe {
        let mut i: i32 = 0;
        let mut res: f64 = 0.0f64;
        i = s - 1;
        while i >= 0 {
            res = res * x + *coeffs.offset(i as isize);
            i -= 1;
            i;
        }
        return res;
    }
}

fn main_0() -> i32 {
    let mut coeffs: [f64; 4] = [-19.0f64, 7.0f64, -4.0f64, 6.0f64];
    print!(
        "{:5.1}\n",
        horner(
            coeffs.as_mut_ptr(),
            (::core::mem::size_of::<[f64; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<f64>() as u64) as i32,
            3.0f64,
        )
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
